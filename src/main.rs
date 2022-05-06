use std::fmt;

// https://www.rfc-editor.org/rfc/rfc4627#section-2
#[derive(Debug, PartialEq)]
pub enum Token {
    Null,         // null
    Bool(bool),   // true or false
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

    fn read_char_and_token(&mut self, token: Token) -> Option<Token> {
        self.input.next();
        Some(token)
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        self.skip_whitespaces();
        if let Some(char) = self.input.peek() {
            match char {
                '{' => Ok(self.read_char_and_token(Token::LeftBrace)),
                '}' => Ok(self.read_char_and_token(Token::RightBrace)),
                '[' => Ok(self.read_char_and_token(Token::LeftBracket)),
                ']' => Ok(self.read_char_and_token(Token::RightBracket)),
                ':' => Ok(self.read_char_and_token(Token::Colon)),
                ',' => Ok(self.read_char_and_token(Token::Comma)),
                'n' => self.read_value("null", Token::Null),
                't' => self.read_value("true", Token::Bool(true)),
                'f' => self.read_value("false", Token::Bool(false)),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    fn read_value(&mut self, value: &str, token: Token) -> Result<Option<Token>, LexerError> {
        let cand: String = (0..value.len())
            .map(|_| self.input.next().unwrap())
            .collect();
        if cand == value {
            Ok(Some(token))
        } else {
            Err(LexerError::new(format!(
                "expected '{}', but got {}",
                value, cand,
            )))
        }
    }

    fn skip_whitespaces(&mut self) {
        loop {
            if let Some(c) = self.input.peek() {
                if c.is_whitespace() {
                    self.input.next();
                    continue;
                }
            }
            break;
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
                    assert_eq!(l.input.collect::<String>(), "");
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
        null: ("null", Some(Token::Null)),
        boolean_true: ("true", Some(Token::Bool(true))),
        // boolean_false: ("false", Some(Token::Bool(false))),
    }
}
