use std::sync::Arc;

use booster_sdk::{
    client::loco::{BoosterClient, GripperCommand},
    types::{
        BoosterHandType, CustomTrainedTraj, DexterousFingerParameter, Frame, GripperControlMode,
        GripperMotionParameter, Hand, Posture,
    },
};
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{
    PyBoosterHandType, PyCustomTrainedTraj, PyDanceId, PyDexterousFingerParameter, PyFrame,
    PyGetModeResponse, PyGetRobotInfoResponse, PyGetStatusResponse, PyGripperCommand,
    PyGripperControlMode, PyGripperMode, PyGripperMotionParameter, PyHand, PyHandAction,
    PyLoadCustomTrainedTrajResponse, PyPosture, PyRobotMode, PyTransform, PyWholeBodyDanceId,
    runtime::wait_for_future, to_py_err,
};

#[pyclass(module = "booster_sdk_bindings", name = "BoosterClient", unsendable)]
pub struct PyBoosterClient {
    client: Arc<BoosterClient>,
}

#[pymethods]
impl PyBoosterClient {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            client: Arc::new(BoosterClient::new().map_err(to_py_err)?),
        })
    }

    fn change_mode(&self, py: Python<'_>, mode: PyRobotMode) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.change_mode(mode.into()).await }).map_err(to_py_err)
    }

    fn get_mode(&self, py: Python<'_>) -> PyResult<PyGetModeResponse> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_mode().await })
            .map(Into::into)
            .map_err(to_py_err)
    }

    fn get_status(&self, py: Python<'_>) -> PyResult<PyGetStatusResponse> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_status().await })
            .map(Into::into)
            .map_err(to_py_err)
    }

    fn get_robot_info(&self, py: Python<'_>) -> PyResult<PyGetRobotInfoResponse> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_robot_info().await })
            .map(Into::into)
            .map_err(to_py_err)
    }

    fn move_robot(&self, py: Python<'_>, vx: f32, vy: f32, vyaw: f32) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.move_robot(vx, vy, vyaw).await }).map_err(to_py_err)
    }

    fn rotate_head(&self, py: Python<'_>, pitch: f32, yaw: f32) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.rotate_head(pitch, yaw).await }).map_err(to_py_err)
    }

    fn wave_hand(&self, py: Python<'_>, action: PyHandAction) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.wave_hand(action.into()).await }).map_err(to_py_err)
    }

    fn rotate_head_with_direction(
        &self,
        py: Python<'_>,
        pitch_direction: i32,
        yaw_direction: i32,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move {
            client
                .rotate_head_with_direction(pitch_direction, yaw_direction)
                .await
        })
        .map_err(to_py_err)
    }

    fn lie_down(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.lie_down().await }).map_err(to_py_err)
    }

    fn get_up(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_up().await }).map_err(to_py_err)
    }

    fn get_up_with_mode(&self, py: Python<'_>, mode: PyRobotMode) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.get_up_with_mode(mode.into()).await },
        )
        .map_err(to_py_err)
    }

    fn shoot(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.shoot().await }).map_err(to_py_err)
    }

    fn push_up(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.push_up().await }).map_err(to_py_err)
    }

    fn move_hand_end_effector_with_aux(
        &self,
        py: Python<'_>,
        target_posture: PyPosture,
        aux_posture: PyPosture,
        time_millis: i32,
        hand_index: PyHand,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let target_posture: Posture = target_posture.into();
        let aux_posture: Posture = aux_posture.into();
        let hand_index: Hand = hand_index.into();
        wait_for_future(py, async move {
            client
                .move_hand_end_effector_with_aux(
                    &target_posture,
                    &aux_posture,
                    time_millis,
                    hand_index,
                )
                .await
        })
        .map_err(to_py_err)
    }

    fn move_hand_end_effector(
        &self,
        py: Python<'_>,
        target_posture: PyPosture,
        time_millis: i32,
        hand_index: PyHand,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let target_posture: Posture = target_posture.into();
        let hand_index: Hand = hand_index.into();
        wait_for_future(py, async move {
            client
                .move_hand_end_effector(&target_posture, time_millis, hand_index)
                .await
        })
        .map_err(to_py_err)
    }

    fn move_hand_end_effector_v2(
        &self,
        py: Python<'_>,
        target_posture: PyPosture,
        time_millis: i32,
        hand_index: PyHand,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let target_posture: Posture = target_posture.into();
        let hand_index: Hand = hand_index.into();
        wait_for_future(py, async move {
            client
                .move_hand_end_effector_v2(&target_posture, time_millis, hand_index)
                .await
        })
        .map_err(to_py_err)
    }

    fn stop_hand_end_effector(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_hand_end_effector().await }).map_err(to_py_err)
    }

    fn control_gripper(
        &self,
        py: Python<'_>,
        motion_param: PyGripperMotionParameter,
        mode: PyGripperControlMode,
        hand_index: PyHand,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let motion_param: GripperMotionParameter = motion_param.into();
        let mode: GripperControlMode = mode.into();
        let hand_index: Hand = hand_index.into();
        wait_for_future(py, async move {
            client.control_gripper(motion_param, mode, hand_index).await
        })
        .map_err(to_py_err)
    }

    fn get_frame_transform(
        &self,
        py: Python<'_>,
        src: PyFrame,
        dst: PyFrame,
    ) -> PyResult<PyTransform> {
        let client = Arc::clone(&self.client);
        let src: Frame = src.into();
        let dst: Frame = dst.into();
        wait_for_future(
            py,
            async move { client.get_frame_transform(src, dst).await },
        )
        .map(Into::into)
        .map_err(to_py_err)
    }

    fn switch_hand_end_effector_control_mode(
        &self,
        py: Python<'_>,
        switch_on: bool,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move {
            client
                .switch_hand_end_effector_control_mode(switch_on)
                .await
        })
        .map_err(to_py_err)
    }

    fn handshake(&self, py: Python<'_>, action: PyHandAction) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.handshake(action.into()).await }).map_err(to_py_err)
    }

    fn control_dexterous_hand(
        &self,
        py: Python<'_>,
        finger_params: Vec<PyDexterousFingerParameter>,
        hand_index: PyHand,
        hand_type: PyBoosterHandType,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let finger_params: Vec<DexterousFingerParameter> =
            finger_params.into_iter().map(Into::into).collect();
        let hand_index: Hand = hand_index.into();
        let hand_type: BoosterHandType = hand_type.into();
        wait_for_future(py, async move {
            client
                .control_dexterous_hand(&finger_params, hand_index, hand_type)
                .await
        })
        .map_err(to_py_err)
    }

    fn control_dexterous_hand_default(
        &self,
        py: Python<'_>,
        finger_params: Vec<PyDexterousFingerParameter>,
        hand_index: PyHand,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let finger_params: Vec<DexterousFingerParameter> =
            finger_params.into_iter().map(Into::into).collect();
        let hand_index: Hand = hand_index.into();
        wait_for_future(py, async move {
            client
                .control_dexterous_hand_default(&finger_params, hand_index)
                .await
        })
        .map_err(to_py_err)
    }

    fn dance(&self, py: Python<'_>, dance_id: PyDanceId) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.dance(dance_id.into()).await }).map_err(to_py_err)
    }

    fn play_sound(&self, py: Python<'_>, sound_file_path: String) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.play_sound(sound_file_path).await })
            .map_err(to_py_err)
    }

    fn stop_sound(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_sound().await }).map_err(to_py_err)
    }

    fn zero_torque_drag(&self, py: Python<'_>, active: bool) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.zero_torque_drag(active).await }).map_err(to_py_err)
    }

    fn record_trajectory(&self, py: Python<'_>, active: bool) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.record_trajectory(active).await })
            .map_err(to_py_err)
    }

    fn replay_trajectory(&self, py: Python<'_>, traj_file_path: String) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.replay_trajectory(traj_file_path).await },
        )
        .map_err(to_py_err)
    }

    fn whole_body_dance(&self, py: Python<'_>, dance_id: PyWholeBodyDanceId) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.whole_body_dance(dance_id.into()).await },
        )
        .map_err(to_py_err)
    }

    fn upper_body_custom_control(&self, py: Python<'_>, start: bool) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.upper_body_custom_control(start).await },
        )
        .map_err(to_py_err)
    }

    fn reset_odometry(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.reset_odometry().await }).map_err(to_py_err)
    }

    fn load_custom_trained_traj(
        &self,
        py: Python<'_>,
        traj: PyCustomTrainedTraj,
    ) -> PyResult<PyLoadCustomTrainedTrajResponse> {
        let client = Arc::clone(&self.client);
        let traj: CustomTrainedTraj = traj.into();
        wait_for_future(
            py,
            async move { client.load_custom_trained_traj(&traj).await },
        )
        .map(Into::into)
        .map_err(to_py_err)
    }

    fn activate_custom_trained_traj(&self, py: Python<'_>, tid: String) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.activate_custom_trained_traj(tid).await },
        )
        .map_err(to_py_err)
    }

    fn unload_custom_trained_traj(&self, py: Python<'_>, tid: String) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.unload_custom_trained_traj(tid).await },
        )
        .map_err(to_py_err)
    }

    fn enter_wbc_gait(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.enter_wbc_gait().await }).map_err(to_py_err)
    }

    fn exit_wbc_gait(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.exit_wbc_gait().await }).map_err(to_py_err)
    }

    fn publish_gripper_command(&self, command: PyGripperCommand) -> PyResult<()> {
        let command: GripperCommand = command.into();
        self.client
            .publish_gripper_command(&command)
            .map_err(to_py_err)
    }

    fn publish_gripper(
        &self,
        hand: PyHand,
        mode: PyGripperMode,
        motion_param: u16,
        speed: Option<u16>,
    ) -> PyResult<()> {
        let command = GripperCommand {
            hand: hand.into(),
            mode: mode.into(),
            motion_param,
            speed: speed.unwrap_or(500),
        };
        self.client
            .publish_gripper_command(&command)
            .map_err(to_py_err)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBoosterClient>()?;
    Ok(())
}
