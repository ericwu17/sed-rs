#[derive(Debug)]
pub enum Command {
    Substitute { regexp: String, replacement: String },
}

pub fn parse_script(script: &str) -> Result<Command, ()> {
    if script.is_empty() {
        return Err(());
    }
    match script.chars().nth(0) {
        None => {
            return Err(());
        }
        Some('s') => {
            let delimiter: char = script.chars().nth(1).ok_or(())?;

            let mut remainder = script.chars().skip(2);
            let mut regexp = String::new();
            let mut replacement = String::new();

            loop {
                // match the search expression
                let c = remainder.next();
                match c {
                    Some(character) => {
                        if character == delimiter {
                            break;
                        }
                        regexp += &character.to_string();
                    }
                    None => return Err(()),
                }
            }
            loop {
                // match the replacement expression
                let c = remainder.next();
                match c {
                    Some(character) => {
                        if character == delimiter {
                            break;
                        }
                        replacement += &character.to_string();
                    }
                    None => return Err(()),
                }
            }

            loop {
                // match additional flags (not yet implemented)
                let c = remainder.next();
                match c {
                    Some(_) => {
                        return Err(());
                    }
                    None => break,
                }
            }

            return Ok(Command::Substitute {
                regexp,
                replacement,
            });
        }
        _ => {
            return Err(());
        }
    }
}
