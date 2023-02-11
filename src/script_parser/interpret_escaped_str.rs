use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum EscapeError {
    EscapeAtEndOfString,
    InvalidEscapedChar(char),
}

struct InterpretEscapedString<'a> {
    s: Chars<'a>,
}

impl<'a> Iterator for InterpretEscapedString<'a> {
    type Item = Result<char, EscapeError>;

    fn next(&mut self) -> Option<Self::Item> {
        let maybe_c = self.s.next();
        match maybe_c {
            Some(c) => match c {
                '\\' => match self.s.next() {
                    Some('t') => Some(Ok('\t')),
                    Some('n') => Some(Ok('\n')),
                    Some('\\') => Some(Ok('\\')),
                    Some(c) => Some(Err(EscapeError::InvalidEscapedChar(c))),
                    None => Some(Err(EscapeError::EscapeAtEndOfString)),
                },
                _ => Some(Ok(c)),
            },
            None => None,
        }
    }
}

pub fn interpret_escaped_string(s: &str) -> Result<String, EscapeError> {
    (InterpretEscapedString { s: s.chars() }).collect()
}
