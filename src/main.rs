extern crate sha1;

mod parse;
mod program_mode;
mod settings;
mod util;

use parse::FromArgsError::InvalidHash;

fn main() {
    match parse::from_args() {
        Ok((settings, program_mode)) => program_mode.run(&settings),
        Err(InvalidHash(_)) => println!("-1"),
        Err(e) => panic!(e)
    }
}
