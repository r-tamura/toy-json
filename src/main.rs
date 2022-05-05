use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {}

pub struct Lexer<'a> {
    input: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&self) -> Result<Option<Token>, LexerError> {
        Ok(None)
    }
}

#[derive(Debug, PartialEq)]
pub struct LexerError(String);

impl LexerError {
    pub fn new(message: impl Into<String>) -> Self {
        LexerError(message.into())
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lexer error")
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_is_not_token() {
        let l = Lexer::new("");
        let actual = l.next_token();
        assert_eq!(actual, Ok(None));
    }
}
