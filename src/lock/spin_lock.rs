use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock<T> {
    /// The data protected by this lock
    ///
    /// Note that `UnsafeCell<T>` is the only legal way to obtain aliasable data
    /// that is considered mutable
    data: UnsafeCell<T>,
    /// True if the lock is taken
    lock_taken: AtomicBool,
}

impl<T> SpinLock<T> {
    /// Return a released `SpinLock`
    pub fn new(data: T) -> SpinLock<T> {
        SpinLock {
            data: UnsafeCell::new(data),
            lock_taken: AtomicBool::new(false)
        }
    }

    /// Execute the given closure after acquiring the lock, and release the
    /// lock afterwards.
    pub fn lock<F: Fn(&mut T)>(&self, f: F) {
        self.take_lock();

        // Obtaining a `&mut T` from `UnsafeCell<T>` is unsafe because the compiler
        // cannot verify that the obtained reference is unique. In other words,
        // it would be possible to obtain multiple mutable references to the same
        // data, at the same time, which would violate Rust's ownership rules and
        // result in undefined behavior.
        //
        // In this case, we know that this is the only reference to the data, because
        // you need to acquire the lock before getting here.
        let data = unsafe { &mut *self.data.get() };
        f(data);

        self.release_lock();
    }

    fn take_lock(&self) {
        // Note that this function could be optimized further (TTaS instead of TaS)

        // Spin while the lock is taken
        // In each iteration, try to take the lock
        // CaS will only swap the values if the previous one was false
        while self.lock_taken.compare_and_swap(false, true, Ordering::SeqCst) {}
    }

    fn release_lock(&self) {
        self.lock_taken.store(false, Ordering::SeqCst);
    }
}

unsafe impl<T: Send> Sync for SpinLock<T> {}
unsafe impl<T: Send> Send for SpinLock<T> {}
