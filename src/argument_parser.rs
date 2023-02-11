use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(name = "sedrs")]
#[command(author = "Eric Wu")]
#[command(version = "0.01")]
#[command(about = "stream editor", long_about = None)]
pub struct Args {
    /// Script to run on the input file
    pub script: String,

    /// Path to input file
    pub input_file: String,
}
