pub mod interpret_escaped_str;
use self::interpret_escaped_str::interpret_escaped_string;

#[derive(Debug)]
pub enum Command {
    Substitute {
        regexp: String,
        replacement: String,
        max_replacements: Option<usize>,
    },
}

pub fn parse_script(script: &str) -> Result<Command, ()> {
    let mut script_iterator = script.chars();

    if script.is_empty() {
        return Err(());
    }

    let first_char = script_iterator.next().ok_or(())?;

    let delimiter: char = script_iterator.next().ok_or(())?;

    let mut regexp = String::new();
    let mut replacement = String::new();

    loop {
        // match the search expression
        let c = script_iterator.next();
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
        let c = script_iterator.next();
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
    let mut max_replacements: Option<usize> = Some(1);

    // match additional flags (only options are 'g' for global or a whole number)
    let remainder: String = script_iterator.collect();
    if remainder == "g" {
        max_replacements = None;
    } else if let Ok(n) = remainder.parse::<usize>() {
        max_replacements = Some(n);
    } else if !remainder.is_empty() {
        return Err(());
    }

    let regexp = interpret_escaped_string(&regexp).map_err(|_| ())?;
    let replacement = interpret_escaped_string(&replacement).map_err(|_| ())?;

    match first_char {
        's' => Ok(Command::Substitute {
            regexp,
            replacement,
            max_replacements,
        }),
        _ => Err(()),
    }
}
