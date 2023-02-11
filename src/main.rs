pub mod argument_parser;
pub mod script_executor;
pub mod script_parser;
use argument_parser::Args;
use clap::Parser;
use script_executor::run_script;
use script_parser::parse_script;

use std::fs;
use std::process;

use std::io::{self, BufRead, BufReader};

fn main() {
    let args = Args::parse();

    let mut rdr: Box<dyn BufRead> = match args.input_file.as_ref() {
        Some(file_path) => Box::new(BufReader::new(fs::File::open(file_path).unwrap_or_else(
            |_| {
                eprintln!("Unable to open file: {}", args.input_file.unwrap());
                process::exit(1)
            },
        ))),
        None => Box::new(BufReader::new(io::stdin())),
    };

    let script = parse_script(&args.script).unwrap_or_else(|()| {
        eprintln!("invalid command: {}", args.script);
        process::exit(1);
    });

    run_script(&mut rdr, script)
}
