#![feature(conservative_impl_trait)]
#![feature(optin_builtin_traits)]

extern crate rand;
extern crate rayon;
extern crate sha1;

mod backend;
mod parse;
mod settings;
mod util;
mod spin_lock_advanced;

use rand::distributions::{Range, IndependentSample};

use backend::Backend;
use parse::FromArgsError::InvalidHash;
use settings::{Mode, Settings};

fn main() {
    match parse::from_args() {
        Ok((settings, mode)) => {
            // NOTE: uncomment the desired backend below
            // -----

            //let backend = backend::SequentialBackend;
            //let backend = RayonBackend::new(settings.threads as usize);
            //let backend = backend::SpinLockBackend::new(settings.threads as usize);
            let backend = backend::AdvancedSpinLockBackend::new(settings.threads as usize);
            //let backend = backend::ThreadBackend;
            run(backend, &settings, mode);
        }
        Err(InvalidHash(_)) => println!("-1"),
        Err(e) => panic!(e)
    }
}

/// This function is not intended to be run. Instead, it provides a reference
/// of how to run the program using the different backends. And it ensures that
/// everything keeps compiling after the program is modified (unlike comments).
#[allow(dead_code)]
fn run_any_backend(settings: &Settings, mode: Mode) {
    use backend::*;
    let mut rng = rand::thread_rng();
    let range = Range::new(0, 5);

    match range.ind_sample(&mut rng) {
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
