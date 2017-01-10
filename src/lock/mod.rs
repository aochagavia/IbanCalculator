//! This module contains the implementation of two spinlocks:
//!
//! The `SpinLock` variant is easiest to implement and to verify for correctness,
//! since the only invariant that needs to be ensured is that there is no mutable
//! aliasing happening at the same time.
//!
//! The `AdvancedSpinLock` variant is harder to implement. Instead of taking a
//! closure, it hands out a reference to the data, wrapped in a `SpinLockGuard`
//! so the lock can be released when the guard is dropped. This requires
//! getting the lifetimes right in order to avoid dangling pointers. There is
//! also slightly more unsafe code than in `SpinLock`.

mod advanced_spin_lock;
mod spin_lock;

pub use self::advanced_spin_lock::AdvancedSpinLock;
pub use self::spin_lock::SpinLock;
