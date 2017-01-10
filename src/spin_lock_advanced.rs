use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::ops::{Deref, DerefMut};
use std::marker;

pub struct SpinLock<T: ?Sized> {
    lock: Box<AtomicBool>,
    data: UnsafeCell<T>,
}
unsafe impl<T: ?Sized + Send> Send for SpinLock<T> { }
unsafe impl<T: ?Sized + Send> Sync for SpinLock<T> { }

pub struct SpinLockGuard<'a, T: ?Sized + 'a> {
    __spin_lock: &'a SpinLock<T>,
}

impl<'a, T: ?Sized> !marker::Send for SpinLockGuard<'a, T> { }

impl<T> SpinLock<T> {
    /// Creates a new SpinLock in an unlocked state ready for use.
    pub fn new(t: T) -> SpinLock<T> {
        SpinLock {
            lock: Box::new(AtomicBool::new(false)),
            data: UnsafeCell::new(t),
        }
    }
}

impl<T: ?Sized> SpinLock<T> {
    /// Acquires a SpinLockGuard, spinning the current thread until it is able to do so.
    pub fn lock(&self) -> SpinLockGuard<T> {
        while self.lock.compare_and_swap(false, true, Ordering::SeqCst) {
            while self.lock.load(Ordering::SeqCst) { }
        }
        // Exit the spinning wait, holding the lock
        unsafe { SpinLockGuard::new(self) }
    }
}

impl<'a, T: ?Sized> Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.__spin_lock.data.get() }
    }
}

impl<'a, T: ?Sized> SpinLockGuard<'a, T> {
    unsafe fn new(lock: &'a SpinLock<T>) -> SpinLockGuard<'a, T> {
        SpinLockGuard {
            __spin_lock: lock,
        }
    }
}
impl<'a, T: ?Sized> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.__spin_lock.data.get() }
    }
}

impl<'a, T: ?Sized> Drop for SpinLockGuard<'a, T> {
    #[inline]
    fn drop(&mut self) {
        self.__spin_lock.lock.store(false, Ordering::SeqCst);
    }
}