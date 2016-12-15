use std::io::Write;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::thread;

use sha1::Sha1;

use settings::Settings;
use util;

use super::Backend;

pub struct ThreadBackend(());

impl ThreadBackend {
    pub fn new() -> ThreadBackend {
        ThreadBackend(())
    }
}

fn split_ranges(low: u32, high: u32, chunks: u32) -> impl Iterator<Item=impl Iterator<Item=u32> + Debug> {
    let delta = (high - low) / chunks;
    (0..chunks).map(move |i| {
        if i == chunks - 1 {
            low + delta * i..high
        } else {
            low + delta * i..low + delta * (1+i)
        }
    })
}

impl Backend for ThreadBackend {
    fn run_count(settings: &Settings) -> u32 {
        // For all x: bottom <= x < top
        //        and m_proef(x, modulo)
        // Count the numer of element satisfying the predicate
        let mut threads = vec![];

        for range in split_ranges(settings.bottom, settings.top, settings.threads) {
            // Spin up another thread
            let modulo = settings.modulo;
            threads.push(thread::spawn(move || {
                let mut count = 0;
                for x in range {
                    if util::m_proef(x, modulo) {
                        count += 1;
                    }
                }
                count
            }));
        }

        let mut count = 0;
        for thread in threads {
            count += thread.join().unwrap();
        }
        count
    }

    fn run_list(settings: &Settings) {
        // For all x: bottom <= x < top
        //        and m_proef(x, modulo)
        // Print the count and the number
        let mut threads = vec![];
        let mutex = Arc::new(Mutex::new(1));

        for range in split_ranges(settings.bottom, settings.top, settings.threads) {
            // Spin up another thread
            let modulo = settings.modulo;
            let mutex = mutex.clone();
            threads.push(thread::spawn(move || {
                for x in range {
                    if util::m_proef(x, modulo) {
                        let mut counter = mutex.lock().unwrap();
                        println!("{} {}", *counter, x);
                        *counter += 1;
                    }
                }
            }));
        }

        for thread in threads {
            thread.join();
        }
    }

    fn run_search(settings: &Settings, hash: Box<[u8; 20]>) -> Option<u32> {
        // FIXME: THIS IS SEQUENTIAL!

        // For all x: bottom <= x < top
        //        and m_proef(x, modulo)
        // Find an x such that sha1(x) == hash
        let mut threads = vec![];

        for range in split_ranges(settings.bottom, settings.top, settings.threads) {
            // Spin up another thread
            let modulo = settings.modulo;
            threads.push(thread::spawn(move || {
                for x in range {
                    if util::m_proef(x, modulo) && util::valid_hash(x, &hash) {
                        return Some(x);
                    }
                }
            }));
        }

        for thread in threads {
            match thread.join() {
                Some(x) => return Some(x),
                None => {},
            }
        }

        None
    }
}
