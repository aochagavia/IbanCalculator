use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::ops::{Deref, DerefMut};

pub struct AdvancedSpinLock<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}
unsafe impl<T: Send> Send for AdvancedSpinLock<T> { }
unsafe impl<T: Send> Sync for AdvancedSpinLock<T> { }

impl<T> AdvancedSpinLock<T> {
    /// Creates a new AdvancedSpinLock in an unlocked state ready for use.
    pub fn new(t: T) -> AdvancedSpinLock<T> {
        AdvancedSpinLock {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(t),
        }
    }

    /// Acquires a SpinLockGuard, spinning the current thread until it is able to do so.
    pub fn lock(&self) -> SpinLockGuard<T> {
        while self.lock.compare_and_swap(false, true, Ordering::SeqCst) {
            while self.lock.load(Ordering::SeqCst) { }
        }

        // Exit the spinning wait, holding the lock
        unsafe { SpinLockGuard::new(self) }

        // Note: the lock will be released by `SpinLockGuard` when it falls out of scope
    }
}

pub struct SpinLockGuard<'a, T: 'a> {
    __spin_lock: &'a AdvancedSpinLock<T>,
}

impl<'a, T> SpinLockGuard<'a, T> {
    unsafe fn new(lock: &'a AdvancedSpinLock<T>) -> SpinLockGuard<'a, T> {
        SpinLockGuard {
            __spin_lock: lock,
        }
    }
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    #[inline]
    fn drop(&mut self) {
        self.__spin_lock.lock.store(false, Ordering::SeqCst);
    }
}

// The `Deref` and `DerefMut` traits are not necessary to enforce safety, but
// are very convenient from an API perspective
impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.__spin_lock.data.get() }
    }
}

impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.__spin_lock.data.get() }
    }
}
