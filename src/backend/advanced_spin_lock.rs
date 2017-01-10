use rayon::{self, Configuration};
use rayon::prelude::*;

use settings::Settings;
use spin_lock_advanced::SpinLock;
use util;

use super::Backend;
use super::rayon::RayonBackend;

pub struct AdvancedSpinLockBackend(());

impl AdvancedSpinLockBackend {
    pub fn new(num_threads: usize) -> AdvancedSpinLockBackend {
        rayon::initialize(Configuration::new().set_num_threads(num_threads)).unwrap();
        AdvancedSpinLockBackend(())
    }
}

impl Backend for AdvancedSpinLockBackend {
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

        let spin_lock = SpinLock::new(1);

        range.filter(|&x| util::m_proef(x, modulo)).for_each(|x| {
            let mut counter = spin_lock.lock();
            println!("{} {}", *counter, x);
            *counter += 1;
        });
    }
}
