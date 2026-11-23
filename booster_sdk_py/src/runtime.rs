use std::{
    future::Future,
    sync::{
        Mutex, MutexGuard, OnceLock, TryLockError,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use pyo3::{PyResult, Python, exceptions::PyRuntimeError};
use tokio::runtime::Runtime;

/// How long a blocking call waits between checks for Python signals
/// (so Ctrl-C is noticed while waiting).
pub(crate) const SIGNAL_CHECK_INTERVAL: Duration = Duration::from_millis(100);

/// Utility to get the Tokio Runtime from Python
#[inline]
pub(crate) fn get_tokio_runtime() -> &'static Runtime {
    // NOTE: Other pyo3 python libraries have had issues with using tokio
    // behind a forking app-server like `gunicorn`
    // If we run into that problem, in the future we can look to `delta-rs`
    // which adds a check in that disallows calls from a forked process
    // https://github.com/delta-io/delta-rs/blob/87010461cfe01563d91a4b9cd6fa468e2ad5f283/python/src/utils.rs#L10-L31
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime"))
}

/// Run `f` to completion on the shared runtime with the GIL released.
///
/// `f` should do very little work besides spawning tasks and awaiting them.
/// It is not interrupted by Ctrl-C (the signal is handled once it returns);
/// use [`wait_for_future_interruptible`] for calls that can run for long.
///
/// See [this] for more information.
///
/// [this]: https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#non-worker-future
#[tracing::instrument(level = "trace", skip_all)]
pub fn wait_for_future<F>(py: Python<'_>, f: F) -> F::Output
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let runtime: &Runtime = get_tokio_runtime();
    py.detach(|| runtime.block_on(f))
}

/// Run `f` to completion with the GIL released, checking for Python signals
/// every [`SIGNAL_CHECK_INTERVAL`].
///
/// If a signal handler raises (e.g. `KeyboardInterrupt` on Ctrl-C), `f` is
/// dropped (cancelled at its current await point) and the error is returned.
pub(crate) fn wait_for_future_interruptible<F>(py: Python<'_>, f: F) -> PyResult<F::Output>
where
    F: Future + Send,
    F::Output: Send,
{
    let runtime: &Runtime = get_tokio_runtime();
    let mut f = std::pin::pin!(f);
    loop {
        let slice = py.detach(|| {
            runtime.block_on(async {
                // `timeout` must be created inside the runtime context.
                tokio::time::timeout(SIGNAL_CHECK_INTERVAL, f.as_mut()).await
            })
        });
        if let Ok(output) = slice {
            return Ok(output);
        }
        py.check_signals()?;
    }
}

/// A mutex for state shared between Python threads whose holder may block
/// with the GIL released.
///
/// [`lock`](Self::lock) never blocks while holding the GIL (a holder waiting
/// to re-acquire the GIL would deadlock), and long-running holders call
/// [`yield_to_waiters`](Self::yield_to_waiters) between time slices so other
/// threads (e.g. one calling `close()`) are not starved.
pub(crate) struct DetachedMutex<T> {
    inner: Mutex<T>,
    waiters: AtomicUsize,
}

impl<T> DetachedMutex<T> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            inner: Mutex::new(value),
            waiters: AtomicUsize::new(0),
        }
    }

    pub(crate) fn lock(&self, py: Python<'_>) -> PyResult<MutexGuard<'_, T>> {
        let mut registered = false;
        let result = loop {
            match self.inner.try_lock() {
                Ok(guard) => break Ok(guard),
                Err(TryLockError::WouldBlock) => {
                    if !registered {
                        self.waiters.fetch_add(1, Ordering::SeqCst);
                        registered = true;
                    }
                    py.detach(|| std::thread::sleep(Duration::from_millis(1)));
                }
                Err(TryLockError::Poisoned(_)) => {
                    break Err(PyRuntimeError::new_err("lock poisoned"));
                }
            }
        };
        if registered {
            self.waiters.fetch_sub(1, Ordering::SeqCst);
        }
        result
    }

    /// Call without holding the lock: if other threads wait for it, pause
    /// (GIL released) until they got it or a few milliseconds passed.
    pub(crate) fn yield_to_waiters(&self, py: Python<'_>) {
        if self.waiters.load(Ordering::SeqCst) == 0 {
            return;
        }
        let waiters = &self.waiters;
        py.detach(|| {
            for _ in 0..20 {
                std::thread::sleep(Duration::from_millis(1));
                if waiters.load(Ordering::SeqCst) == 0 {
                    break;
                }
            }
        });
    }
}
