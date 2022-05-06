use std::fmt;

// https://www.rfc-editor.org/rfc/rfc4627#section-2
#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,    // '{'
    RightBrace,   // '}'
    LeftBracket,  // '['
    RightBracket, // ']'
    Colon,        // ':'
    Comma,        // ','
}

pub struct Lexer<'a> {
    input: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        if let Some(char) = self.input.peek() {
            let token = match char {
                '{' => Token::LeftBrace,
                '}' => Token::RightBrace,
                '[' => Token::LeftBracket,
                ']' => Token::RightBracket,
                ':' => Token::Colon,
                ',' => Token::Comma,
                _ => return Ok(None),
            };
            Ok(Some(token))
        } else {
            Ok(None)
        }
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
        write!(f, "Lexer error: {}", self.0)
    }
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_next_token {
        ($( $name:ident: ($token:expr, $expected:expr),)*) => {
            $(
                #[test]
                fn $name() {
                    let mut l = Lexer::new($token);
                    let actual = l.next_token();
                    assert_eq!(actual, Ok($expected));
                }
            )*
        }
    }

    test_next_token! {
        empty_string: ("", None),
        white_space: (" ", None),
        left_brace: ("{", Some(Token::LeftBrace)),
        right_brace: ("}", Some(Token::RightBrace)),
        left_bracket: ("[", Some(Token::LeftBracket)),
        right_bracket: ("]", Some(Token::RightBracket)),
        colon: (":", Some(Token::Colon)),
        comma: (",", Some(Token::Comma)),
    }
}
