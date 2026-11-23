//! Type-erased Python wrapper around [`DdsSubscription`].
//!
//! Every DDS subscription exposed to Python returns a [`PySubscription`]
//! (`booster_sdk_bindings.Subscription`). Bindings build one with
//! [`PySubscription::new`], passing a function that converts each Rust sample
//! into its Python class.

// Constructors are used by the client bindings as they add subscriptions.
#![allow(dead_code)]

use std::sync::Mutex;
use std::time::{Duration, Instant};

use booster_sdk::dds::DdsSubscription;
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::runtime::get_tokio_runtime;

/// How long a blocking `recv` waits between checks for Python signals
/// (so Ctrl-C works while waiting).
const SIGNAL_CHECK_INTERVAL: Duration = Duration::from_millis(100);

type Converter<T> = Box<dyn Fn(Python<'_>, T) -> PyResult<Py<PyAny>> + Send>;

/// Outcome of waiting for one time slice.
enum Slice {
    Sample(Py<PyAny>),
    Pending,
    Closed,
}

trait ErasedSubscription: Send {
    /// Wait up to `slice` for one sample (without holding the GIL).
    fn recv_slice(&mut self, py: Python<'_>, slice: Duration) -> PyResult<Slice>;
    fn try_recv(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>>;
    fn latest(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>>;
    fn matched_publications(&self) -> usize;
    fn close(&mut self);
}

struct Typed<T> {
    subscription: DdsSubscription<T>,
    convert: Converter<T>,
}

impl<T: Send + 'static> ErasedSubscription for Typed<T> {
    fn recv_slice(&mut self, py: Python<'_>, slice: Duration) -> PyResult<Slice> {
        let subscription = &mut self.subscription;
        let waited = py.detach(|| {
            get_tokio_runtime()
                .block_on(async { tokio::time::timeout(slice, subscription.recv()).await })
        });
        match waited {
            Ok(Some(value)) => Ok(Slice::Sample((self.convert)(py, value)?)),
            Ok(None) => Ok(Slice::Closed),
            Err(_elapsed) => Ok(Slice::Pending),
        }
    }

    fn try_recv(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.subscription
            .try_recv()
            .map(|value| (self.convert)(py, value))
            .transpose()
    }

    fn latest(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.subscription
            .latest()
            .map(|value| (self.convert)(py, value))
            .transpose()
    }

    fn matched_publications(&self) -> usize {
        self.subscription.matched_publications()
    }

    fn close(&mut self) {
        self.subscription.close();
    }
}

/// A DDS topic subscription.
///
/// `recv(timeout=None)` blocks (releasing the GIL) until a sample arrives,
/// `try_recv()` returns immediately, and iterating yields samples forever.
/// Each returns `None` once the subscription is closed (or on timeout).
#[pyclass(module = "booster_sdk_bindings", name = "Subscription")]
pub struct PySubscription {
    inner: Mutex<Option<Box<dyn ErasedSubscription>>>,
}

impl PySubscription {
    /// Wrap a subscription, converting each sample with `convert`.
    pub(crate) fn new<T, F>(subscription: DdsSubscription<T>, convert: F) -> Self
    where
        T: Send + 'static,
        F: Fn(Python<'_>, T) -> PyResult<Py<PyAny>> + Send + 'static,
    {
        Self {
            inner: Mutex::new(Some(Box::new(Typed {
                subscription,
                convert: Box::new(convert),
            }))),
        }
    }

    /// Wrap a subscription whose samples convert with `Into<P>` into a pyclass `P`.
    pub(crate) fn of<T, P>(subscription: DdsSubscription<T>) -> Self
    where
        T: Send + 'static,
        P: From<T> + pyo3::PyClass + Into<pyo3::PyClassInitializer<P>>,
    {
        Self::new(subscription, |py, value: T| {
            Ok(Py::new(py, P::from(value))?.into_any())
        })
    }

    fn with_inner<R>(
        &self,
        f: impl FnOnce(&mut Box<dyn ErasedSubscription>) -> PyResult<Option<R>>,
    ) -> PyResult<Option<R>> {
        let mut guard = self
            .inner
            .lock()
            .map_err(|_| PyValueError::new_err("subscription lock poisoned"))?;
        match guard.as_mut() {
            Some(inner) => f(inner),
            None => Ok(None),
        }
    }
}

#[pymethods]
impl PySubscription {
    /// Block until a sample arrives, or `timeout` seconds pass (`None` = forever).
    #[pyo3(signature = (timeout=None))]
    fn recv(&self, py: Python<'_>, timeout: Option<f64>) -> PyResult<Option<Py<PyAny>>> {
        let deadline = match timeout {
            Some(seconds) if !seconds.is_finite() || seconds < 0.0 => {
                return Err(PyValueError::new_err(
                    "timeout must be a finite number >= 0",
                ));
            }
            Some(seconds) => Some(Instant::now() + Duration::from_secs_f64(seconds)),
            None => None,
        };
        loop {
            let slice = deadline.map_or(SIGNAL_CHECK_INTERVAL, |deadline| {
                deadline
                    .saturating_duration_since(Instant::now())
                    .min(SIGNAL_CHECK_INTERVAL)
            });
            match self.with_inner(|inner| inner.recv_slice(py, slice).map(Some))? {
                Some(Slice::Sample(sample)) => return Ok(Some(sample)),
                Some(Slice::Pending) => {}
                // Closed, or `close()` was called.
                Some(Slice::Closed) | None => return Ok(None),
            }
            if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
                return Ok(None);
            }
            py.check_signals()?;
        }
    }

    /// Return the next queued sample, or `None` if none is pending.
    fn try_recv(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.with_inner(|inner| inner.try_recv(py))
    }

    /// Drain the queue and return only the newest sample, or `None`.
    fn latest(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.with_inner(|inner| inner.latest(py))
    }

    /// Number of remote publishers currently matched with this subscription.
    fn matched_publications(&self) -> PyResult<usize> {
        Ok(self
            .with_inner(|inner| Ok(Some(inner.matched_publications())))?
            .unwrap_or(0))
    }

    /// Stop receiving and release the DDS reader.
    fn close(&self) -> PyResult<()> {
        let mut guard = self
            .inner
            .lock()
            .map_err(|_| PyValueError::new_err("subscription lock poisoned"))?;
        if let Some(mut inner) = guard.take() {
            inner.close();
        }
        Ok(())
    }

    /// Whether `close()` has been called.
    fn is_closed(&self) -> PyResult<bool> {
        Ok(self
            .inner
            .lock()
            .map_err(|_| PyValueError::new_err("subscription lock poisoned"))?
            .is_none())
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        // `None` ends iteration (subscription closed).
        self.recv(py, None)
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    #[pyo3(signature = (*_args))]
    fn __exit__(&self, _args: &Bound<'_, pyo3::types::PyTuple>) -> PyResult<()> {
        self.close()
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySubscription>()
}
