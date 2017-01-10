//! This module implements different backends that solve the `IbanCalculator`
//! assignment. See the documentation for the `Backend` trait for more details.

mod sequential;
mod rayon;
mod spin_lock;
mod advanced_spin_lock;
mod threads;

use settings::Settings;

pub use self::sequential::SequentialBackend;
pub use self::rayon::RayonBackend;
pub use self::spin_lock::SpinLockBackend;
pub use self::advanced_spin_lock::AdvancedSpinLockBackend;
pub use self::threads::ThreadBackend;

/// The `Backend` trait provides functions to execute the three program modes
/// from the `IbanCalculator` assignment.
pub trait Backend {
    fn run_count(settings: &Settings) -> u32;
    fn run_list(settings: &Settings);
    fn run_search(settings: &Settings, hash: Box<[u8; 20]>) -> Option<u32>;
}
