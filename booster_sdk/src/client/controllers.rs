//! Convenience controllers ported from the C++ SDK (`move_controller.hpp`,
//! `arm_controller.hpp`).
//!
//! - [`MoveController`] drives the robot to odometry targets using
//!   fire-and-forget velocity commands and the `rt/odometer_state` topic.
//! - [`ArmController`] interpolates the arm joints (indices 2–9) through
//!   low-level `rt/joint_ctrl` commands while upper-body custom control is on.
//!
//! Both mirror the C++ behaviour, including its timings and gains. Their
//! motion loops have no overall timeout (like the C++ versions): wrap calls in
//! [`tokio::time::timeout`] if you need one. Dropping a future mid-motion stops
//! sending commands; call [`MoveController::stop`] / [`ArmController::close`]
//! to leave the robot in a defined state.

use std::collections::HashMap;
use std::f32::consts::PI;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::dds::{
    DdsCallbackSubscription, DdsPublisher, LowCmd, LowState, MotorCmd, Odometer, RpcClientOptions,
    low_state_topic, odometer_state_topic,
};
use crate::types::{BoosterError, CmdType, JOINT_CNT, RPC_STATUS_CODE_TIMEOUT, Result};

use super::BoosterClient;

const CONTROL_PERIOD: Duration = Duration::from_millis(20);
const ARM_CONTROL_PERIOD: Duration = Duration::from_millis(10);

/// Status code of an RPC failure, if the error came from an RPC.
fn rpc_status(err: &BoosterError) -> Option<i32> {
    match err {
        BoosterError::Rpc(rpc) => Some(rpc.status_code()),
        _ => None,
    }
}

/// Retry `call` while it fails with RPC status 100 (timeout), like the C++
/// `RetryApiCall`. Any other error aborts immediately.
async fn retry_api_call<F, Fut>(
    description: &str,
    max_retries: u32,
    delay: Duration,
    mut call: F,
) -> Result<()>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<()>>,
{
    let mut last_err = None;
    for attempt in 1..=max_retries {
        match call().await {
            Ok(()) => return Ok(()),
            Err(err) if rpc_status(&err) == Some(RPC_STATUS_CODE_TIMEOUT) => {
                tracing::warn!(
                    "'{description}' timed out (status 100), retry {attempt}/{max_retries}"
                );
                last_err = Some(err);
                tokio::time::sleep(delay).await;
            }
            Err(err) => {
                tracing::error!("'{description}' failed: {err}");
                return Err(err);
            }
        }
    }
    Err(last_err.unwrap_or_else(|| BoosterError::Other(format!("'{description}' failed"))))
}

/// Run a cleanup future from `Drop`: on the current tokio runtime if there is
/// one, otherwise on a short-lived background thread with its own runtime.
fn run_detached<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        handle.spawn(future);
        return;
    }
    let spawned = std::thread::Builder::new()
        .name("booster-controller-cleanup".to_owned())
        .spawn(move || {
            match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime.block_on(future),
                Err(err) => tracing::warn!("controller cleanup runtime failed: {err}"),
            }
        });
    if let Err(err) = spawned {
        tracing::warn!("controller cleanup thread failed: {err}");
    }
}

fn normalize_angle(mut angle: f32) -> f32 {
    const TWO_PI: f32 = 2.0 * PI;
    while angle > PI {
        angle -= TWO_PI;
    }
    while angle < -PI {
        angle += TWO_PI;
    }
    angle
}

/// Poll `ready` every 10 ms until it returns true or `timeout` elapses.
async fn wait_until(timeout: Duration, ready: impl Fn() -> bool) -> bool {
    let start = Instant::now();
    while !ready() {
        if start.elapsed() > timeout {
            return false;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    true
}

// ---------------------------------------------------------------------------
// MoveController
// ---------------------------------------------------------------------------

/// Odometry-based point-to-point locomotion (C++ `b1::MoveController`).
///
/// Requires the robot to be in a walking mode.
pub struct MoveController {
    client: Arc<BoosterClient>,
    pose: Arc<Mutex<Option<Odometer>>>,
    odometer: Option<DdsCallbackSubscription>,
    closed: bool,
}

impl MoveController {
    /// Create a controller with default client options.
    pub async fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::default()).await
    }

    /// Create a controller with explicit client options (network, robot name, ...).
    pub async fn with_options(options: RpcClientOptions) -> Result<Self> {
        Self::with_client(Arc::new(BoosterClient::with_options(options)?)).await
    }

    /// Create a controller on an existing client.
    ///
    /// Like the C++ constructor this waits up to 5 s for odometry (failing if
    /// none arrives), waits 2 s, then resets odometry (retrying RPC timeouts up
    /// to 5 times). A failed reset is logged, not returned, as in C++.
    pub async fn with_client(client: Arc<BoosterClient>) -> Result<Self> {
        let pose = Arc::new(Mutex::new(None));
        let sink = Arc::clone(&pose);
        let odometer =
            client
                .node()
                .subscribe_callback(&odometer_state_topic(), move |odom: Odometer| {
                    *sink.lock().expect("pose lock poisoned") = Some(odom);
                })?;

        let mut controller = Self {
            client,
            pose,
            odometer: Some(odometer),
            closed: false,
        };

        let ready = {
            let pose = Arc::clone(&controller.pose);
            wait_until(Duration::from_secs(5), move || {
                pose.lock().expect("pose lock poisoned").is_some()
            })
            .await
        };
        if !ready {
            controller.odometer = None;
            controller.closed = true;
            return Err(BoosterError::Other("Odometer not responding.".to_owned()));
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
        let client = Arc::clone(&controller.client);
        if let Err(err) = retry_api_call("ResetOdometry", 5, Duration::from_secs(1), || {
            client.reset_odometry()
        })
        .await
        {
            tracing::error!("ResetOdometry failed after retries: {err}");
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
        Ok(controller)
    }

    /// The underlying locomotion client.
    pub fn client(&self) -> &Arc<BoosterClient> {
        &self.client
    }

    /// Latest odometry pose `(x, y, theta)`.
    pub fn pose(&self) -> (f32, f32, f32) {
        let odom = self
            .pose
            .lock()
            .expect("pose lock poisoned")
            .unwrap_or_default();
        (odom.x, odom.y, odom.theta)
    }

    /// Walk to the odometry-frame target `(x, y)` with speed limit `vel` (m/s).
    ///
    /// First turns toward the target (up to 5 s, skipped within 0.6 m), then
    /// translates while holding heading, until within 0.20 m.
    pub async fn move_to_target(&self, x: f32, y: f32, vel: f32) {
        const DISTANCE_TOLERANCE: f32 = 0.20;

        let rotation_start = Instant::now();
        loop {
            let (cur_x, cur_y, cur_theta) = self.pose();
            let dist = (x - cur_x).hypot(y - cur_y);
            if dist < 0.6 || rotation_start.elapsed() > Duration::from_secs(5) {
                break;
            }
            let target_heading = (y - cur_y).atan2(x - cur_x);
            let heading_error = normalize_angle(target_heading - cur_theta);
            if heading_error.abs() < 0.2 {
                break;
            }
            let vyaw = (1.2 * heading_error).clamp(-1.0, 1.0);
            self.safe_move(0.0, 0.0, vyaw).await;
            tokio::time::sleep(CONTROL_PERIOD).await;
        }
        self.safe_move(0.0, 0.0, 0.0).await;

        let hold_theta = self.pose().2;
        loop {
            let (cur_x, cur_y, cur_theta) = self.pose();
            let dx = x - cur_x;
            let dy = y - cur_y;
            let dist = dx.hypot(dy);
            if dist < DISTANCE_TOLERANCE {
                self.stop().await;
                return;
            }

            let (sin_theta, cos_theta) = cur_theta.sin_cos();
            let mut vx = dx * cos_theta + dy * sin_theta;
            let mut vy = -dx * sin_theta + dy * cos_theta;

            let mut local_limit = vel;
            if dist < 0.2 {
                local_limit = dist.max(0.1);
                if dist < 0.1 {
                    local_limit = local_limit.max(0.05);
                }
            }
            let current_speed = vx.hypot(vy);
            if current_speed > local_limit && current_speed > 1e-6 {
                let scale = local_limit / current_speed;
                vx *= scale;
                vy *= scale;
            }
            let vyaw = normalize_angle(hold_theta - cur_theta).clamp(-0.5, 0.5);
            self.safe_move(vx, vy, vyaw).await;
            tokio::time::sleep(CONTROL_PERIOD).await;
        }
    }

    /// Walk `(dx, dy)` relative to the current odometry position.
    pub async fn move_to_relative(&self, dx: f32, dy: f32, vel: f32) {
        let (cur_x, cur_y, _) = self.pose();
        self.move_to_target(cur_x + dx, cur_y + dy, vel).await;
    }

    /// Turn in place by `|angle|` radians at `|vel|` rad/s.
    ///
    /// Note: the turn direction follows the C++ SDK, where a positive `angle`
    /// commands a negative yaw rate.
    pub async fn turn_around(&self, angle: f32, vel: f32) {
        let direction_sign = if angle > 0.0 { -1.0 } else { 1.0 };
        let target_accumulated = angle.abs();
        let vel_mag = vel.abs();
        let mut current_accumulated = 0.0_f32;
        let mut last_theta = self.pose().2;

        loop {
            let curr_theta = self.pose().2;
            current_accumulated += normalize_angle(curr_theta - last_theta).abs();
            last_theta = curr_theta;

            let remaining = target_accumulated - current_accumulated;
            if remaining <= 0.0 {
                self.stop().await;
                return;
            }
            let cmd_vel = if remaining < 0.2 {
                (remaining * 2.0).max(0.2)
            } else {
                vel_mag
            };
            self.safe_move(0.0, 0.0, direction_sign * cmd_vel).await;
            tokio::time::sleep(CONTROL_PERIOD).await;
        }
    }

    /// Command zero velocity.
    pub async fn stop(&self) {
        self.safe_move(0.0, 0.0, 0.0).await;
    }

    /// Stop the robot and close the odometry subscription.
    pub async fn close(&mut self) {
        if self.closed {
            return;
        }
        self.stop().await;
        self.odometer = None;
        self.closed = true;
    }

    /// Send a velocity command; RPC timeouts (dropped packets) and other
    /// failures are logged and ignored, like the C++ `SafeMove`.
    async fn safe_move(&self, vx: f32, vy: f32, vyaw: f32) {
        match self.client.move_command(vx, vy, vyaw).await {
            Ok(()) => {}
            Err(err) if rpc_status(&err) == Some(RPC_STATUS_CODE_TIMEOUT) => {
                tracing::warn!("move command send timeout (status 100), continuing");
            }
            Err(err) => tracing::error!("move command failed: {err}"),
        }
    }
}

impl Drop for MoveController {
    fn drop(&mut self) {
        if self.closed {
            return;
        }
        // Best effort: send a stop without blocking the dropping thread.
        let client = Arc::clone(&self.client);
        run_detached(async move {
            let _ = client.move_command(0.0, 0.0, 0.0).await;
        });
    }
}

// ---------------------------------------------------------------------------
// ArmController
// ---------------------------------------------------------------------------

crate::api_id_enum! {
    /// Arm joints controllable by [`ArmController`] (C++ `b1::ArmJoint`).
    ArmJoint {
        LeftPitch = 2,
        LeftRoll = 3,
        LeftYaw = 4,
        LeftElbow = 5,
        RightPitch = 6,
        RightRoll = 7,
        RightYaw = 8,
        RightElbow = 9,
    }
}

const ARM_JOINT_START: usize = 2;
const ARM_JOINT_END: usize = 9;
const ARM_MOTOR_MODE: u8 = 0x0A;
const ARM_KP: f32 = 60.0;
const ARM_KD: f32 = 3.0;

#[derive(Debug, Clone, Copy)]
struct PendingAction {
    target_rad: f32,
    duration_seconds: f32,
}

#[derive(Debug, Clone, Copy)]
struct PlannedTask {
    start_q: f32,
    end_q: f32,
    duration_seconds: f32,
}

#[derive(Default)]
struct JointTracker {
    ready: bool,
    q: Vec<f32>,
}

/// Interpolated arm-joint control via low-level commands (C++ `b1::ArmController`).
///
/// Queue targets with [`control_arm`](Self::control_arm) and execute them with
/// [`finish`](Self::finish). Upper-body custom control is enabled on creation
/// and disabled by [`close`](Self::close).
pub struct ArmController {
    client: Arc<BoosterClient>,
    publisher: Option<DdsPublisher<LowCmd>>,
    low_state: Option<DdsCallbackSubscription>,
    tracker: Arc<Mutex<JointTracker>>,
    joint_count: usize,
    pending: HashMap<usize, PendingAction>,
    closed: bool,
}

impl ArmController {
    /// Create a controller with default client options and the T1 joint layout.
    pub async fn new() -> Result<Self> {
        Self::with_client(
            Arc::new(BoosterClient::with_options(RpcClientOptions::default())?),
            JOINT_CNT,
        )
        .await
    }

    /// Create a controller on an existing client.
    ///
    /// `joint_count` is the low-level layout size (the C++ class always uses
    /// the 23-joint T1 layout, [`JOINT_CNT`]; use
    /// [`crate::types::JOINT_CNT_K1`] for a K1). Like the C++ constructor, this
    /// waits up to 5 s for `rt/low_state` (warning on timeout), waits 2 s,
    /// enables upper-body custom control (retrying RPC timeouts 5 times; a
    /// failure is logged) and waits 500 ms.
    pub async fn with_client(client: Arc<BoosterClient>, joint_count: usize) -> Result<Self> {
        let publisher = client.low_cmd_publisher()?;
        let tracker = Arc::new(Mutex::new(JointTracker {
            ready: false,
            q: vec![0.0; joint_count],
        }));
        let sink = Arc::clone(&tracker);
        let low_state =
            client
                .node()
                .subscribe_callback(&low_state_topic(), move |state: LowState| {
                    let mut tracker = sink.lock().expect("joint tracker lock poisoned");
                    for (slot, motor) in tracker.q.iter_mut().zip(&state.motor_state_parallel) {
                        *slot = motor.q;
                    }
                    tracker.ready = true;
                })?;

        let controller = Self {
            client,
            publisher: Some(publisher),
            low_state: Some(low_state),
            tracker,
            joint_count,
            pending: HashMap::new(),
            closed: false,
        };

        let tracker = Arc::clone(&controller.tracker);
        if !wait_until(Duration::from_secs(5), move || {
            tracker.lock().expect("joint tracker lock poisoned").ready
        })
        .await
        {
            tracing::warn!("LowState timeout");
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
        let client = Arc::clone(&controller.client);
        if let Err(err) = retry_api_call(
            "Enable UpperBodyCustomControl",
            5,
            Duration::from_secs(1),
            || client.upper_body_custom_control(true),
        )
        .await
        {
            tracing::error!("failed to enable UpperBodyCustomControl after retries: {err}");
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
        Ok(controller)
    }

    /// The underlying locomotion client.
    pub fn client(&self) -> &Arc<BoosterClient> {
        &self.client
    }

    /// Latest measured joint positions (parallel layout).
    pub fn joint_positions(&self) -> Vec<f32> {
        self.tracker
            .lock()
            .expect("joint tracker lock poisoned")
            .q
            .clone()
    }

    /// Queue a move of `joint` to `target_rad` over `duration_ms` (min 50 ms).
    ///
    /// Chainable; nothing moves until [`finish`](Self::finish).
    pub fn control_arm(&mut self, joint: ArmJoint, target_rad: f32, duration_ms: f32) -> &mut Self {
        self.control_arm_index(i32::from(joint), target_rad, duration_ms)
    }

    /// Like [`control_arm`](Self::control_arm) with a raw joint index;
    /// indices outside 2–9 are ignored.
    pub fn control_arm_index(
        &mut self,
        joint_idx: i32,
        target_rad: f32,
        duration_ms: f32,
    ) -> &mut Self {
        let Ok(idx) = usize::try_from(joint_idx) else {
            return self;
        };
        if !(ARM_JOINT_START..=ARM_JOINT_END).contains(&idx) {
            return self;
        }
        self.pending.insert(
            idx,
            PendingAction {
                target_rad,
                duration_seconds: duration_ms.max(50.0) / 1000.0,
            },
        );
        self
    }

    /// Execute the queued moves, linearly interpolating from the current
    /// positions at 100 Hz. Arm joints without a queued move hold their
    /// measured position; other joints get zero gains.
    ///
    /// Returns the first publish error, if any (publishing continues on error,
    /// like the C++ version).
    pub async fn finish(&mut self) -> Result<()> {
        if self.pending.is_empty() {
            return Ok(());
        }
        let Some(publisher) = self.publisher.as_ref() else {
            return Err(BoosterError::Other("ArmController is closed".to_owned()));
        };

        let current = self.joint_positions();
        let mut max_duration = 0.0_f32;
        let tasks: HashMap<usize, PlannedTask> = self
            .pending
            .iter()
            .map(|(&idx, action)| {
                max_duration = max_duration.max(action.duration_seconds);
                let task = PlannedTask {
                    start_q: current.get(idx).copied().unwrap_or(0.0),
                    end_q: action.target_rad,
                    duration_seconds: action.duration_seconds,
                };
                (idx, task)
            })
            .collect();

        let mut first_err = None;
        let start = Instant::now();
        loop {
            let elapsed = start.elapsed().as_secs_f32();
            if elapsed > max_duration + 0.02 {
                break;
            }
            let real_q = self.joint_positions();
            let mut cmd = LowCmd::new(CmdType::default(), self.joint_count);
            for (i, motor) in cmd.motor_cmd.iter_mut().enumerate() {
                *motor = MotorCmd {
                    mode: ARM_MOTOR_MODE,
                    ..MotorCmd::default()
                };
                if let Some(task) = tasks.get(&i) {
                    let progress = (elapsed / task.duration_seconds).min(1.0);
                    motor.q = task.start_q + (task.end_q - task.start_q) * progress;
                    motor.kp = ARM_KP;
                    motor.kd = ARM_KD;
                } else if (ARM_JOINT_START..=ARM_JOINT_END).contains(&i) {
                    motor.q = real_q.get(i).copied().unwrap_or(0.0);
                    motor.kp = ARM_KP;
                    motor.kd = ARM_KD;
                }
            }
            if let Err(err) = publisher.write(cmd) {
                first_err.get_or_insert(err);
            }
            tokio::time::sleep(ARM_CONTROL_PERIOD).await;
        }

        self.pending.clear();
        first_err.map_or(Ok(()), Err)
    }

    /// Disable upper-body custom control (retrying RPC timeouts 3 times) and
    /// close the channels.
    pub async fn close(&mut self) {
        if self.closed {
            return;
        }
        let client = Arc::clone(&self.client);
        if let Err(err) = retry_api_call(
            "Disable UpperBodyCustomControl",
            3,
            Duration::from_secs(1),
            || client.upper_body_custom_control(false),
        )
        .await
        {
            tracing::warn!("failed to disable UpperBodyCustomControl: {err}");
        }
        self.low_state = None;
        self.publisher = None;
        self.pending.clear();
        self.closed = true;
    }
}

impl Drop for ArmController {
    fn drop(&mut self) {
        if self.closed {
            return;
        }
        // Best effort: disable upper-body custom control without blocking.
        let client = Arc::clone(&self.client);
        run_detached(async move {
            let _ = client.upper_body_custom_control(false).await;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_angle_wraps_into_pi_range() {
        assert!((normalize_angle(3.0 * PI) - PI).abs() < 1e-5);
        assert!((normalize_angle(-3.0 * PI) + PI).abs() < 1e-5);
        assert!((normalize_angle(0.5) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn arm_joint_ids_match_cpp() {
        assert_eq!(i32::from(ArmJoint::LeftPitch), 2);
        assert_eq!(i32::from(ArmJoint::RightElbow), 9);
    }
}
