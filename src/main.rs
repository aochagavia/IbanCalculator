#![feature(conservative_impl_trait)]
#![feature(optin_builtin_traits)]

extern crate rand;
extern crate rayon;
extern crate sha1;

mod backend;
mod lock;
mod parse;
mod settings;
mod util;

use rand::distributions::{Range, IndependentSample};

use backend::Backend;
use parse::FromArgsError::InvalidHash;
use settings::{Mode, Settings};

/// The entry point of the appllication
fn main() {
    match parse::from_args() {
        Ok((settings, mode)) => run_any_backend(&settings, mode),
        Err(InvalidHash(_)) => println!("-1"),
        Err(e) => panic!(e)
    }
}

/// Run the `IbanCalculator` assignment, with the given backend, settings and mode
pub fn run<T: Backend>(_imp: T, settings: &Settings, mode: Mode) {
    use self::Mode::*;
    match mode {
        Count => println!("{}", T::run_count(settings)),
        List => T::run_list(settings),
        Search(hash) => match T::run_search(settings, hash) {
            Some(x) => println!("{}", x),
            None => println!("-1")
        }
    }
}

/// Run the `IbanCalculator` assignment using any backend
fn run_any_backend(settings: &Settings, mode: Mode) {
    use backend::*;
    let mut rng = rand::thread_rng();
    let range = Range::new(0, 5);

    // Note: in case you want to run a particular backend, replace the backend index
    // by the desired integer.
    let backend_index = range.ind_sample(&mut rng);
    match backend_index {
        0 => {
            let backend = SpinLockBackend::new(settings.threads as usize);
            run(backend, settings, mode)
        }
        1 => {
            let backend = RayonBackend::new(settings.threads as usize);
            run(backend, settings, mode)
        }
        2 => {
            let backend = AdvancedSpinLockBackend::new(settings.threads as usize);
            run(backend, settings, mode)
        }
        3 => run(ThreadBackend, settings, mode),
        4 => run(SequentialBackend, settings, mode),
        _ => unreachable!(),
    }
}
