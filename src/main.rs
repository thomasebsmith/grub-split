use std::error::Error;
use std::io;
use std::process::ExitCode;

mod run;

fn invalid_input(desc: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, desc)
}

fn usage(args: &[String]) -> io::Error {
    if args.is_empty() {
        invalid_input(String::from("Usage: <executable> <pid>"))
    } else {
        invalid_input(format!("Usage: {} <pid>", args[0]))
    }
}

fn parse_args_and_run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(Box::new(usage(args.as_slice())));
    }

    let pid: i32 = args[1].parse()?;

    run::run(pid)
}

fn main() -> ExitCode {
    env_logger::init();
    match parse_args_and_run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {}", error);
            ExitCode::FAILURE
        }
    }
}
