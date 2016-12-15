use sha1::Sha1;

use settings::Settings;
use util;

use super::ProgramMode;

pub enum SequentialMode {}

impl ProgramMode for SequentialMode {
    fn run_count(settings: &Settings) -> u32 {
        // For all x: bottom <= x < top
        //        and m_proef(x, modulo)
        // Count the numer of element satisfying the predicate
        let range = settings.bottom .. settings.top;
        let mut counter = 0;
        for x in range {
            if util::m_proef(x, settings.modulo) {
                counter += 1;
            }
        }

        counter
    }

    fn run_list(settings: &Settings) {
        // For all x: bottom <= x < top
        //        and m_proef(x, modulo)
        // Print the count and the number
        let range = settings.bottom .. settings.top;
        let mut counter = 1;
        for x in range {
            if util::m_proef(x, settings.modulo) {
                println!("{} {}", counter, x);

                counter += 1;
            }
        }
    }

    fn run_search(settings: &Settings, hash: Box<[u8; 20]>) -> Option<u32> {
        // For all x: bottom <= x < top
        //        and m_proef(x, modulo)
        // Find an x such that sha1(x) == hash
        let mut sha1 = Sha1::new();
        let mut buffer: Vec<u8> = Vec::with_capacity(9);
        (settings.bottom .. settings.top)
            .find(|&x| util::m_proef(x, settings.modulo)
                    && util::valid_hash_fast(x, &hash, &mut buffer, &mut sha1))
    }
}
