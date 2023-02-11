use crate::script_parser::Command;
use std::io::{self, BufRead, Write};

pub fn run_script(input_file: &mut Box<dyn BufRead>, script: Command) {
    match script {
        Command::Substitute {
            regexp,
            replacement,
            max_replacements,
        } => {
            let mut line = String::new();
            while input_file.read_line(&mut line).unwrap() != 0 {
                write_bytes_with_replacements(
                    line.as_bytes(),
                    regexp.as_bytes(),
                    replacement.as_bytes(),
                    max_replacements,
                );
                line = String::new();
            }
            io::stdout().flush().unwrap()
        }
    }
}

fn write_bytes_with_replacements(
    bytes: &[u8],
    search_for: &[u8],
    replacement: &[u8],
    max_replacements: Option<usize>,
) {
    let mut prev_written_index = 0;
    let mut num_replacements_performed = 0;
    let window_size: usize = search_for.len();

    if window_size > bytes.len() || max_replacements == Some(0) {
        io::stdout().write_all(bytes).unwrap();
        return;
    }

    for i in 0..(bytes.len() - window_size + 1) {
        if &bytes[i..(i + window_size)] == search_for {
            io::stdout()
                .write_all(&bytes[prev_written_index..i])
                .unwrap();
            io::stdout().write_all(replacement).unwrap();
            prev_written_index = i + window_size;
            num_replacements_performed += 1;
        }
        if let Some(n) = max_replacements {
            if num_replacements_performed >= n {
                break;
            }
        }
    }

    io::stdout()
        .write_all(&bytes[prev_written_index..])
        .unwrap();
}
