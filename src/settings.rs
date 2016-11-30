use std::env;
use std::num::ParseIntError;

#[derive(Debug)]
enum ProgramMode {
    Count,
    List,
    Search(String),
}

#[derive(Debug)]
pub struct Settings {
    custom_lock: bool,
    bottom: u32,
    top: u32,
    modulo: u32,
    threads: u32,
    program_mode: ProgramMode
}

#[derive(Debug)]
pub enum FromArgsError {
    InvalidArgumentAmount(usize),
    InvalidLockNumber(i32),
    InvalidProgramMode(i32),
    ParseError(&'static str, ParseIntError)
}

impl Settings {
    pub fn from_args() -> Result<Settings, FromArgsError> {
        let args: Vec<_> = env::args().collect();

        if args.len() != 7 && args.len() != 8 {
            return Err(InvalidArgumentAmount(args.len()))
        }

        use self::FromArgsError::*;
        let settings = Settings {
            custom_lock: match args[1].parse() {
                Ok(0) => true,
                Ok(1) => false,
                Ok(n) => return Err(InvalidLockNumber(n)),
                Err(e) => return Err(ParseError("Lock", e))
            },
            bottom: match args[2].parse() {
                Ok(n) => n,
                Err(e) => return Err(ParseError("Bottom", e))
            },
            top: match args[3].parse() {
                Ok(n) => n,
                Err(e) => return Err(ParseError("Top", e))
            },
            modulo: match args[4].parse() {
                Ok(n) => n,
                Err(e) => return Err(ParseError("Modulo", e))
            },
            threads: match args[5].parse() {
                Ok(n) => n,
                Err(e) => return Err(ParseError("Threads", e))
            },
            program_mode: match args[6].parse() {
                Ok(0) => ProgramMode::Count,
                Ok(1) => ProgramMode::List,
                Ok(2) => ProgramMode::Search(args[7].to_string()),
                Ok(n) => return Err(InvalidProgramMode(n)),
                Err(e) => return Err(ParseError("Program mode", e))
            }
        };

        Ok(settings)
    }
}
