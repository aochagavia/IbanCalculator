use std::sync::atomic::{AtomicBool, Ordering};

use rayon::prelude::*;

use super::Backend;
use super::rayon::RayonBackend;

use settings::Settings;
use util;

struct SpinLock {
    // true if the lock is taken
    lock_taken: AtomicBool
}

impl SpinLock {
    fn new() -> SpinLock {
        SpinLock {
            lock_taken: AtomicBool::new(false)
        }
    }

    fn lock<F: Fn()>(&self, f: F) {
        // Spin while the lock is taken
        // In each iteration, try to take the lock
        // CaS will only swap the values if the previous one was false
        loop {
            let is_taken = self.lock_taken.compare_and_swap(false, true, Ordering::SeqCst);

            // If the lock was not taken, it means that CaS just took it for us,
            // so we can go ahead and execute our function
            if !is_taken {
                f();

                // Release the lock and exit the loop
                self.lock_taken.store(false, Ordering::SeqCst);
                break;
            }
        }
    }
}

pub struct SpinLockBackend(RayonBackend);

impl SpinLockBackend {
    pub fn new(num_threads: usize) -> SpinLockBackend {
        SpinLockBackend(RayonBackend::new(num_threads))
    }
}

impl Backend for SpinLockBackend {
    // Redirect run_count and run_search to the RayonBackend, since they don't use locks
    fn run_count(settings: &Settings) -> u32 { RayonBackend::run_count(settings) }
    fn run_search(settings: &Settings, hash: Box<[u8; 20]>) -> Option<u32> {
        RayonBackend::run_search(settings, hash)
    }

    // Implement our own version of run_list
    fn run_list(settings: &Settings) {
        // For all x: bottom <= x < top
        //        and m_proef(x, modulo)
        // Print the count and the number
        let modulo = settings.modulo;
        let range = (settings.bottom .. settings.top).into_par_iter();

        let mut counter = 1;
        let mutex = SpinLock::new();

        range.filter(|&x| util::m_proef(x, modulo)).for_each(|x| {
            mutex.lock(|| {
                println!("{} {}", counter, x);
                counter += 1;
            });
        });
    }
}
