extern crate rayon;
extern crate sha1;

mod modes;
mod parse;
mod settings;
mod util;

use modes::{ProgramMode, RayonMode};
use parse::FromArgsError::InvalidHash;
use settings::{Mode, Settings};

fn main() {
    match parse::from_args() {
        Ok((settings, mode)) => {
            let imp = RayonMode::new(settings.threads as usize);
            run(imp, &settings, mode);
        }
        Err(InvalidHash(_)) => println!("-1"),
        Err(e) => panic!(e)
    }
}

pub fn run<T: ProgramMode>(_imp: T, settings: &Settings, mode: Mode) {
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
