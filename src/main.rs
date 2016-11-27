use std::env;

#[derive(Debug)]
enum Lock {
    TTAS,
    Lock,
}

#[derive(Debug)]
enum ProgramMode {
    Count(Settings),
    List(Settings),
    Search(Settings, String),
}

#[derive(Debug)]
struct Settings {
    lock: Lock,
    bottom: u32,
    exclusive: u32,
    modulo: u32,
    threads: u32,
}

fn main() {
    // Prints each argument on a separate line
    let args = env::args();
    let program_mode: ProgramMode;
    if args.len() == 7 || args.len() == 8 {
        program_mode = get_args(args);
    } else {
        println!("Length: {:?}", args.len());
        panic!("Length: {:?}", args.len());
    }
    println!("{:?}", program_mode);
    println!("{:?}", m_proef(274856190, 11));
}

fn get_args(args: env::Args) -> ProgramMode {
    let args: Vec<String> = args.collect();
    let settings = Settings {
        lock: match args[1].parse() {
            Ok(0) => Lock::TTAS,
            Ok(1) => Lock::Lock,
            Ok(n) => {
                println!("Lock : {:?}", n);
                panic!("Lock : {:?}", n);
            }
            Err(err) => {
                println!("Lock : {:?}", err);
                panic!("Lock : {:?}", err);
            }
        },
        bottom: match args[2].parse() {
            Ok(n) => n,
            Err(err) => {
                println!("Bottom : {:?}", err);
                panic!("Bottom : {:?}", err);
            }
        },
        exclusive: match args[3].parse() {
            Ok(n) => n,
            Err(err) => {
                println!("Exclusive : {:?}", err);
                panic!("Exclusive : {:?}", err);
            }
        },
        modulo: match args[4].parse() {
            Ok(n) => n,
            Err(err) => {
                println!("Modulo : {:?}", err);
                panic!("Modulo : {:?}", err);
            }
        },
        threads: match args[5].parse() {
            Ok(n) => n,
            Err(err) => {
                println!("Threads : {:?}", err);
                panic!("Threads : {:?}", err);
            }
        },
    };
    match args[6].parse() {
        Ok(0) => ProgramMode::Count(settings),
        Ok(1) => ProgramMode::List(settings),
        Ok(2) => ProgramMode::Search(settings, args[7].to_string()),
        Ok(n) => {
            println!("ProgramMode : {:?}", n);
            panic!("ProgramMode : {:?}", n);
        }
        Err(err) => {
            println!("Bottom : {:?}", err);
            panic!("Bottom : {:?}", err);
        }
    }
}

fn m_proef(test: u32, modulo: u32) -> bool {
    let mut rest: u32 = test;
    let mut counter: u32 = 0;
    let mut index: u32 = 1;
    while rest != 0 {
        counter += (rest % 10) * index;
        rest /= 10;
        index += 1;
    }
    (counter % modulo) == 0
}
