#![feature(conservative_impl_trait)]

extern crate rayon;
extern crate sha1;

mod backend;
mod parse;
mod settings;
mod util;

use backend::Backend;
use parse::FromArgsError::InvalidHash;
use settings::{Mode, Settings};

fn main() {
    match parse::from_args() {
        Ok((settings, mode)) => {
            //let backend = RayonBackend::new(settings.threads as usize);
            let backend = backend::SpinLockBackend::new(settings.threads as usize);
            run(backend, &settings, mode);
        }
        Err(InvalidHash(_)) => println!("-1"),
        Err(e) => panic!(e)
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
