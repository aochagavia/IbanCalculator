use std::io::{self, BufRead};
use std::num::ParseIntError;

use program_mode::ProgramMode;
use settings::Settings;
use util;

#[derive(Debug)]
pub enum FromArgsError {
    InvalidArgumentAmount(usize),
    InvalidHash(String),
    InvalidLockNumber(i32),
    InvalidProgramMode(i32),
    ParseError(&'static str, ParseIntError)
}

pub fn from_args() -> Result<(Settings, ProgramMode), FromArgsError> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let line = stdin.lines().next().unwrap().unwrap();
    let args: Vec<_> = line.trim_right().split_whitespace().collect();

    if args.len() != 6 && args.len() != 7 {
        return Err(InvalidArgumentAmount(args.len()))
    }

    use self::FromArgsError::*;
    let settings = Settings {
        custom_lock: match args[0].parse() {
            Ok(0) => true,
            Ok(1) => false,
            Ok(n) => return Err(InvalidLockNumber(n)),
            Err(e) => return Err(ParseError("Lock", e))
        },
        bottom: match args[1].parse() {
            Ok(n) => n,
            Err(e) => return Err(ParseError("Bottom", e))
        },
        top: match args[2].parse() {
            Ok(n) => n,
            Err(e) => return Err(ParseError("Top", e))
        },
        modulo: match args[3].parse() {
            Ok(n) => n,
            Err(e) => return Err(ParseError("Modulo", e))
        },
        threads: match args[4].parse() {
            Ok(n) => n,
            Err(e) => return Err(ParseError("Threads", e))
        }
    };

    let program_mode = match args[5].parse() {
        Ok(0) => ProgramMode::Count,
        Ok(1) => ProgramMode::List,
        Ok(2) => match util::sha1_hex_to_bytes(&args[6]) {
            Some(hash) => ProgramMode::Search(hash),
            None => return Err(InvalidHash(args[6].to_owned()))
        },
        Ok(n) => return Err(InvalidProgramMode(n)),
        Err(e) => return Err(ParseError("Program mode", e))
    };

    Ok((settings, program_mode))
}