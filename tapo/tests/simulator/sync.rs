//! Locking the state that the simulated devices share between requests.

use std::sync::{Mutex, MutexGuard, PoisonError};

/// The state behind `mutex`, even after a panic while it was locked: the
/// panic names the reason for one request, and a poisoned lock would hide
/// that behind a `PoisonError` in every request that follows.
pub fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}
