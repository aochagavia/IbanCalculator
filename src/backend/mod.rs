mod sequential;
mod rayon;
mod threads;

use settings::Settings;

pub use self::rayon::RayonBackend;
pub use self::sequential::SequentialBackend;
pub use self::threads::ThreadBackend;

pub trait Backend {
    fn run_count(settings: &Settings) -> u32;
    fn run_list(settings: &Settings);
    fn run_search(settings: &Settings, hash: Box<[u8; 20]>) -> Option<u32>;
}