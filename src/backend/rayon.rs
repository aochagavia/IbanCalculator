use std::sync::Mutex;

use rayon::{self, Configuration};
use rayon::prelude::*;

use settings::Settings;
use util;

use super::Backend;

pub struct RayonBackend(());

impl RayonBackend {
    pub fn new(num_threads: usize) -> RayonBackend {
        rayon::initialize(Configuration::new().set_num_threads(num_threads)).unwrap();
        RayonBackend(())
    }
}

impl Backend for RayonBackend {
    fn run_count(settings: &Settings) -> u32 {
        // For all x: bottom <= x < top
        //        and m_proef(x, modulo)
        // Count the numer of element satisfying the predicate
        let modulo = settings.modulo;
        let range = (settings.bottom .. settings.top).into_par_iter();
        range.filter(|&x| util::m_proef(x, modulo)).count() as u32
    }

    fn run_list(settings: &Settings) {
        // For all x: bottom <= x < top
        //        and m_proef(x, modulo)
        // Print the count and the number
        let modulo = settings.modulo;
        let range = (settings.bottom .. settings.top).into_par_iter();

        let mutex = Mutex::new(1);

        range.filter(|&x| util::m_proef(x, modulo)).for_each(|x| {
            let mut counter = mutex.lock().unwrap();
            println!("{} {}", *counter, x);
            *counter += 1;
        });
    }

    fn run_search(settings: &Settings, hash: Box<[u8; 20]>) -> Option<u32> {
        // For all x: bottom <= x < top
        //        and m_proef(x, modulo)
        // Find an x such that sha1(x) == hash
        (settings.bottom .. settings.top)
            .into_par_iter()
            .find_any(|&x| util::m_proef(x, settings.modulo)
                        && util::valid_hash(x, &hash))
    }
}
