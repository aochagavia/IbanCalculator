use std::io::Write;
use std::sync::Mutex;
use std::sync::mpsc;

use rayon::{self, Configuration};
use rayon::prelude::*;
use sha1::Sha1;

use settings::Settings;
use util;

use super::ProgramMode;

pub struct RayonMode(());

impl RayonMode {
    pub fn new(num_threads: usize) -> RayonMode {
        rayon::initialize(Configuration::new().set_num_threads(num_threads)).unwrap();
        RayonMode(())
    }
}

impl ProgramMode for RayonMode {
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
        // FIXME: THIS IS SEQUENTIAL!
        // Vec<range>
        // Work stealing, as long as nothing has been found

        // For all x: bottom <= x < top
        //        and m_proef(x, modulo)
        // Find an x such that sha1(x) == hash
        let range = settings.bottom .. settings.top;
        let (tx, rx) = mpsc::channel();
        range.into_par_iter().find_any(|&x| {
            if util::m_proef(x, settings.modulo) {
                let mut sha1 = Sha1::new();
                let mut buffer: Vec<u8> = Vec::with_capacity(9);

                // Turn the x into a string (the provided hash is derived from the string,
                // not the number itself)
                buffer.clear();
                write!(buffer, "{}", x).unwrap();

                // Calculate the sha1 and compare
                sha1.reset();
                sha1.update(&buffer);

                if sha1.digest().bytes() == *hash {
                    tx.send(x).unwrap();
                    return true;
                }
            }

            false
        });

        rx.try_recv().ok()
    }
}
