extern crate sha1;

mod parse;
mod program_mode;
mod settings;
mod util;

fn main() {
    let (settings, program_mode) = parse::from_args().expect("Error parsing settings");
    program_mode.run(&settings)
}
