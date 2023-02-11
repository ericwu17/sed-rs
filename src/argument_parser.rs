use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(name = "sedrs")]
#[command(author = "Eric Wu")]
#[command(version = "0.01")]
#[command(about = "stream editor", long_about = None)]
pub struct Args {
    /// Name of the person to greet
    pub script: String,

    /// Number of times to greet
    pub input_file: String,
}
