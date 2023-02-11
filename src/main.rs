pub mod argument_parser;
pub mod script_executor;
pub mod script_parser;
use argument_parser::Args;
use clap::Parser;
use script_executor::run_script;
use script_parser::parse_script;

use std::fs;
use std::process;

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.input_file).unwrap_or_else(|_| {
        println!("Unable to open file: {}", args.input_file);
        process::exit(1);
    });

    let script = parse_script(&args.script).unwrap_or_else(|()| {
        println!("invalid command: {}", args.script);
        process::exit(1);
    });

    run_script(contents, script)
}
