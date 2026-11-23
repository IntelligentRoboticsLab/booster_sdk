//! Asynchronous locomotion operations (`BoosterClient.start_operation`).

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::{Duration, Instant};

use booster_sdk::{
    client::loco::BoosterClient,
    dds::{
        ApiOperation, ApiOperationEvent, ApiOperationEventType, ApiOperationHandle,
        ApiOperationProgress, ApiOperationResult, ApiOperationResultCode,
    },
    types::{BoosterError, RpcError},
};
use pyo3::{Bound, exceptions::PyValueError, prelude::*, types::PyModule};

use crate::{
    macros::py_readonly_class,
    runtime::{DetachedMutex, SIGNAL_CHECK_INTERVAL, get_tokio_runtime, wait_for_future},
    to_py_err,
};

/// Bind a fieldless Rust enum without integer conversions in the core crate.
macro_rules! py_simple_enum {
    (
        $py_name:ident, $public_name:literal, $inner:ty,
        { $($attr:ident => $variant:ident = $value:literal),+ $(,)? }
    ) => {
        #[pyclass(module = "booster_sdk_bindings", name = $public_name, eq, hash, frozen)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $py_name(pub(crate) $inner);

        #[pymethods]
        impl $py_name {
            $(
                #[classattr]
                const $attr: Self = Self(<$inner>::$variant);
            )+

            /// Convert a raw value; raises `ValueError` for unknown values.
            #[staticmethod]
            fn from_int(value: i32) -> PyResult<Self> {
                match value {
                    $($value => Ok(Self(<$inner>::$variant)),)+
                    _ => Err(PyValueError::new_err(format!(
                        concat!("invalid ", $public_name, " value: {}"),
                        value
                    ))),
                }
            }

            #[getter]
            fn name(&self) -> &'static str {
                match self.0 {
                    $(<$inner>::$variant => stringify!($attr),)+
                }
            }

            fn __int__(&self) -> i32 {
                i32::from(self.0)
            }

            fn __index__(&self) -> i32 {
                i32::from(self.0)
            }

            fn __repr__(&self) -> String {
                format!(concat!($public_name, ".{}"), self.name())
            }
        }

        impl From<$inner> for $py_name {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }
    };
}

py_simple_enum!(PyApiOperationEventType, "ApiOperationEventType", ApiOperationEventType, {
    UNKNOWN => Unknown = 0,
    PROGRESS => Progress = 1,
    FINISHED => Finished = 2,
});

py_simple_enum!(PyApiOperationResultCode, "ApiOperationResultCode", ApiOperationResultCode, {
    UNKNOWN => Unknown = 0,
    SUCCEEDED => Succeeded = 1,
    ABORTED => Aborted = 2,
    CANCELED => Canceled = 3,
    REJECTED => Rejected = 4,
    TIMEOUT => Timeout = 5,
});

py_readonly_class!(PyApiOperationProgress, "ApiOperationProgress", ApiOperationProgress, {
    operation_id: String,
    api_id: i64,
    status: i64,
    message: String,
    body: String,
}, {
    /// Always `ApiOperationEventType.PROGRESS`.
    #[getter]
    fn event_type(&self) -> PyApiOperationEventType {
        PyApiOperationEventType(ApiOperationEventType::Progress)
    }
});

py_readonly_class!(PyApiOperationResult, "ApiOperationResult", ApiOperationResult, {
    code: PyApiOperationResultCode,
    operation_id: String,
    api_id: i64,
    status: i64,
    message: String,
    body: String,
}, {
    /// Always `ApiOperationEventType.FINISHED`.
    #[getter]
    fn event_type(&self) -> PyApiOperationEventType {
        PyApiOperationEventType(ApiOperationEventType::Finished)
    }

    /// Whether the operation succeeded (`code == SUCCEEDED` and `status == 0`).
    fn is_success(&self) -> bool {
        self.0.is_success()
    }

    /// Return `body` on success; otherwise raise the matching `RpcError`.
    fn body_or_raise(&self) -> PyResult<String> {
        self.0.clone().into_body().map_err(to_py_err)
    }
});

/// Validate an optional timeout in seconds into a deadline.
fn deadline_from(timeout: Option<f64>) -> PyResult<Option<Instant>> {
    match timeout {
        Some(seconds) if !seconds.is_finite() || seconds < 0.0 => Err(PyValueError::new_err(
            "timeout must be a finite number >= 0",
        )),
        Some(seconds) => Ok(Some(Instant::now() + Duration::from_secs_f64(seconds))),
        None => Ok(None),
    }
}

enum Next {
    Event(ApiOperationEvent),
    Pending,
    Closed,
}

/// A started locomotion operation (`BoosterClient.start_operation`).
///
/// Progress events and the final result are received with `next_event()` /
/// `wait()`; both block with the GIL released and honour Ctrl-C.
/// Dropping the object stops event delivery but does not cancel the
/// operation on the robot: call `cancel()` for that.
#[pyclass(module = "booster_sdk_bindings", name = "ApiOperation")]
pub struct PyApiOperation {
    client: Arc<BoosterClient>,
    handle: ApiOperationHandle,
    inner: DetachedMutex<ApiOperation>,
    finished: AtomicBool,
}

impl PyApiOperation {
    pub(crate) fn handle(&self) -> &ApiOperationHandle {
        &self.handle
    }

    pub(crate) fn new(client: Arc<BoosterClient>, operation: ApiOperation) -> Self {
        Self {
            client,
            handle: operation.handle().clone(),
            inner: DetachedMutex::new(operation),
            finished: AtomicBool::new(false),
        }
    }

    /// Wait up to `slice` for the next event (GIL released).
    fn next_slice(&self, py: Python<'_>, slice: Duration) -> PyResult<Next> {
        let mut operation = self.inner.lock(py)?;
        let operation = &mut *operation;
        let waited = py.detach(|| {
            get_tokio_runtime()
                .block_on(async { tokio::time::timeout(slice, operation.next_event()).await })
        });
        Ok(match waited {
            Ok(Some(event)) => {
                if matches!(event, ApiOperationEvent::Finished(_)) {
                    self.finished.store(true, Ordering::SeqCst);
                }
                Next::Event(event)
            }
            Ok(None) => Next::Closed,
            Err(_elapsed) => Next::Pending,
        })
    }

    /// Next event before `deadline` (`None` = forever), checking signals.
    fn next_until(
        &self,
        py: Python<'_>,
        deadline: Option<Instant>,
    ) -> PyResult<Option<ApiOperationEvent>> {
        loop {
            let slice = deadline.map_or(SIGNAL_CHECK_INTERVAL, |deadline| {
                deadline
                    .saturating_duration_since(Instant::now())
                    .min(SIGNAL_CHECK_INTERVAL)
            });
            match self.next_slice(py, slice)? {
                Next::Event(event) => return Ok(Some(event)),
                Next::Closed => return Ok(None),
                Next::Pending => {}
            }
            if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
                return Ok(None);
            }
            py.check_signals()?;
            self.inner.yield_to_waiters(py);
        }
    }
}

fn event_to_py(py: Python<'_>, event: ApiOperationEvent) -> PyResult<Py<PyAny>> {
    Ok(match event {
        ApiOperationEvent::Progress(progress) => {
            Py::new(py, PyApiOperationProgress::from(progress))?.into_any()
        }
        ApiOperationEvent::Finished(result) => {
            Py::new(py, PyApiOperationResult::from(result))?.into_any()
        }
    })
}

#[pymethods]
impl PyApiOperation {
    #[getter]
    fn operation_id(&self) -> &str {
        &self.handle.operation_id
    }

    #[getter]
    fn api_id(&self) -> i32 {
        self.handle.api_id
    }

    /// Whether the final result has been received.
    #[getter]
    fn done(&self) -> bool {
        self.finished.load(Ordering::SeqCst)
    }

    /// Next `ApiOperationProgress` / `ApiOperationResult`, or `None` on
    /// timeout or once the final result has been delivered.
    #[pyo3(signature = (timeout=None))]
    fn next_event(&self, py: Python<'_>, timeout: Option<f64>) -> PyResult<Option<Py<PyAny>>> {
        let deadline = deadline_from(timeout)?;
        self.next_until(py, deadline)?
            .map(|event| event_to_py(py, event))
            .transpose()
    }

    /// Wait for the final result, skipping progress events.
    ///
    /// Raises `RpcTimeoutError` if `timeout` seconds pass first, and
    /// `RpcStatusError` if the event stream closed without a result (e.g. the
    /// result was already consumed by `next_event`).
    #[pyo3(signature = (timeout=None))]
    fn wait(&self, py: Python<'_>, timeout: Option<f64>) -> PyResult<PyApiOperationResult> {
        let deadline = deadline_from(timeout)?;
        loop {
            match self.next_until(py, deadline)? {
                Some(ApiOperationEvent::Finished(result)) => return Ok(result.into()),
                Some(ApiOperationEvent::Progress(_)) => {}
                None if deadline.is_some_and(|deadline| Instant::now() >= deadline) => {
                    let timeout = Duration::from_secs_f64(timeout.unwrap_or_default());
                    return Err(to_py_err(RpcError::Timeout { timeout }.into()));
                }
                None => {
                    return Err(to_py_err(BoosterError::from(RpcError::RequestFailed {
                        status: -1,
                        message: "operation event stream closed".to_owned(),
                    })));
                }
            }
        }
    }

    /// Ask the service to cancel this operation; the final result (usually
    /// `CANCELED`) still arrives through `wait()` / `next_event()`.
    fn cancel(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let handle = self.handle.clone();
        wait_for_future(py, async move { client.cancel_operation(&handle).await })
            .map_err(to_py_err)
    }

    fn __repr__(&self) -> String {
        format!(
            "ApiOperation(operation_id={:?}, api_id={})",
            self.handle.operation_id, self.handle.api_id
        )
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyApiOperationEventType>()?;
    m.add_class::<PyApiOperationResultCode>()?;
    m.add_class::<PyApiOperationProgress>()?;
    m.add_class::<PyApiOperationResult>()?;
    m.add_class::<PyApiOperation>()?;
    Ok(())
}
