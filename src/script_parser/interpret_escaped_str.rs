#[derive(Debug, PartialEq)]
pub enum EscapeError {
    EscapeAtEndOfString,
    InvalidEscapedChar(char),
}

pub fn interpret_escaped_string(s: &str) -> Result<String, EscapeError> {
    let mut result = String::new();

    let mut s_iterator = s.chars();

    while let Some(char) = s_iterator.next() {
        match char {
            '\\' => match s_iterator.next() {
                Some('t') => result += "\t",
                Some('n') => result += "\n",
                Some('\\') => result += "\\",
                Some(c) => return Err(EscapeError::InvalidEscapedChar(c)),
                None => return Err(EscapeError::EscapeAtEndOfString),
            },
            _ => {
                result += char.to_string().as_str();
            }
        }
    }

    Ok(result)
}
