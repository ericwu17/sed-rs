use crate::script_parser::Command;
use std::io::{self, Write};

pub fn run_script(input_file: String, script: Command) {
    let lines = input_file.split("\n");

    match script {
        Command::Substitute {
            regexp,
            replacement,
        } => {
            let mut is_first_line: bool = true;
            for line in lines {
                if !is_first_line {
                    println!("");
                } else {
                    is_first_line = false;
                }

                write_bytes_with_single_replacement(
                    line.as_bytes(),
                    regexp.as_bytes(),
                    replacement.as_bytes(),
                );
            }
            io::stdout().flush().unwrap()
        }
    }
}

fn write_bytes_with_single_replacement(bytes: &[u8], search_for: &[u8], replacement: &[u8]) {
    let window_size: usize = search_for.len();

    if window_size > bytes.len() {
        io::stdout().write(bytes).unwrap();
        return;
    }

    for i in 0..(bytes.len() - window_size) {
        if &bytes[i..(i + window_size)] == search_for {
            io::stdout().write(&bytes[0..i]).unwrap();
            io::stdout().write(replacement).unwrap();
            io::stdout().write(&bytes[(i + window_size)..]).unwrap();
            return;
        }
    }

    io::stdout().write(bytes).unwrap();
}
