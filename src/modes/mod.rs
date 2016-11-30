mod sequential;
mod rayon;

use settings::Settings;

pub use self::rayon::RayonMode;
pub use self::sequential::SequentialMode;

pub trait ProgramMode {
    fn run_count(settings: &Settings) -> u32;
    fn run_list(settings: &Settings);
    fn run_search(settings: &Settings, hash: Box<[u8; 20]>) -> Option<u32>;
}
