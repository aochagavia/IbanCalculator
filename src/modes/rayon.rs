use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

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
        // FIXME: the counter in the output is not sequential because
        // println uses its own lock.

        // For all x: bottom <= x < top
        //        and m_proef(x, modulo)
        // Print the count and the number
        let modulo = settings.modulo;
        let range = (settings.bottom .. settings.top).into_par_iter();
        let counter = AtomicUsize::new(1);
        range.filter(|&x| util::m_proef(x, modulo)).for_each(|x| {
            let count = counter.fetch_add(1, Ordering::SeqCst);
            println!("{} {}", count, x);
        });
    }

    fn run_search(settings: &Settings, hash: Box<[u8; 20]>) -> Option<u32> {
        // FIXME: THIS IS SEQUENTIAL!

        // For all x: bottom <= x < top
        //        and m_proef(x, modulo)
        // Find an x such that sha1(x) == hash
        let range = settings.bottom .. settings.top;
        let mut sha1 = Sha1::new();
        let mut buffer: Vec<u8> = Vec::with_capacity(9);
        for x in range {
            if util::m_proef(x, settings.modulo) {
                // Turn the x into a string (the provided hash is derived from the string,
                // not the number itself)
                buffer.clear();
                write!(buffer, "{}", x).unwrap();

                // Calculate the sha1 and compare
                sha1.reset();
                sha1.update(&buffer);

                if sha1.digest().bytes() == *hash {
                    return Some(x);
                }
            }
        }

        None
    }
}