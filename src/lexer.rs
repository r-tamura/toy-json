use std::fmt;

// https://www.rfc-editor.org/rfc/rfc4627#section-2
#[derive(Debug, PartialEq)]
pub enum Token {
    Null,        // null
    Bool(bool),  // true or false
    Number(f64), // 42, 3.14, 10e3 ...
    String(String),
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

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        while let Some(tok) = self.next_token()? {
            tokens.push(tok);
        }
        Ok(tokens)
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        self.skip_whitespaces();
        if let Some(char) = self.input.peek() {
            match char {
                '{' => Ok(self.read_char_and_return_token(Token::LeftBrace)),
                '}' => Ok(self.read_char_and_return_token(Token::RightBrace)),
                '[' => Ok(self.read_char_and_return_token(Token::LeftBracket)),
                ']' => Ok(self.read_char_and_return_token(Token::RightBracket)),
                ':' => Ok(self.read_char_and_return_token(Token::Colon)),
                ',' => Ok(self.read_char_and_return_token(Token::Comma)),
                'n' => self.read_keyword("null", Token::Null),
                't' => self.read_keyword("true", Token::Bool(true)),
                'f' => self.read_keyword("false", Token::Bool(false)),
                '-' | '0'..='9' => self.read_number(),
                '"' => self.read_string(),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    fn read_char_and_return_token(&mut self, token: Token) -> Option<Token> {
        assert!(self.input.peek().is_some());
        self.input.next();
        Some(token)
    }

    fn read_keyword(&mut self, value: &str, token: Token) -> Result<Option<Token>, LexerError> {
        let cand = self.read_next_chars(value.len());
        if cand == value {
            Ok(Some(token))
        } else {
            Err(LexerError::new(format!(
                "expected '{}', but got {}",
                value, cand,
            )))
        }
    }

    fn read_number(&mut self) -> Result<Option<Token>, LexerError> {
        let mut cand: String = String::new();
        loop {
            if let Some(c) = self.input.peek() {
                const MINUS: char = '-';
                const PLUS: char = '+';
                const DECIMAL_POINT: char = '.';
                const EXP_LOWERCASE: char = 'e';
                const EXP_UPPERCASE: char = 'E';
                const SYMBOLS: [char; 5] =
                    [MINUS, PLUS, DECIMAL_POINT, EXP_UPPERCASE, EXP_LOWERCASE];

                if '0' <= (*c) && (*c <= '9') || SYMBOLS.contains(c) {
                    // We has already check if the nest char exists
                    cand.push(self.input.next().unwrap());
                    continue;
                }
            }
            break;
        }
        if let Ok(n) = cand.parse::<f64>() {
            Ok(Some(Token::Number(n)))
        } else {
            Err(LexerError::new(format!("couldn't parse number '{}'", cand)))
        }
    }

    fn read_string(&mut self) -> Result<Option<Token>, LexerError> {
        // https://www.rfc-editor.org/rfc/rfc8259#section-7
        assert!(
            self.input.peek().unwrap() == &'"',
            "start double quote of a string"
        );
        self.input.next();
        let mut cand = String::new();

        while let Some(c) = self.input.next_if(|&c| c != '"') {
            if c == '\\' {
                let next_char = self
                    .input
                    .next()
                    .ok_or(LexerError::new(format!("unexpected escaped char")))?;
                match next_char {
                    // 4HEXDIG ex: \u005C
                    'u' => cand.push(self.read_unicode()?),
                    'b' => cand.push('\u{000B}'), // backspace
                    'f' => cand.push('\u{000C}'), // formfeed
                    'n' => cand.push('\n'),
                    'r' => cand.push('\r'),
                    't' => cand.push('\t'),
                    '\\' => cand.push('\\'),
                    _ => return Err(LexerError::new(format!("unexcepted escaped char"))),
                }
            } else {
                cand.push(c);
            }
        }

        assert!(
            self.input.peek().unwrap() == &'"',
            "end double quote of a string"
        );
        self.input.next();
        Ok(Some(Token::String(cand)))
    }

    fn read_unicode(&mut self) -> Result<char, LexerError> {
        let code_point = self.read_next_chars(4);
        let res = u32::from_str_radix(&code_point, 16).map(|code_point| char::from_u32(code_point));
        match res {
            Ok(Some(c)) => Ok(c),
            Ok(None) => Err(LexerError::new(format!(
                "'{}' is not a code point",
                code_point
            ))),
            Err(err) => return Err(LexerError::new(format!("{}", err))),
        }
    }

    fn read_next_chars(&mut self, n: usize) -> String {
        (0..n).filter_map(|_| self.input.next()).collect::<String>()
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
                    assert_eq!(actual, $expected);
                    assert_eq!(l.input.collect::<String>(), "");
                }
            )*
        }
    }

    test_next_token! {
        empty_string: ("", Ok(None)),
        white_space: (" ", Ok(None)),
        left_brace: ("{", Ok(Some(Token::LeftBrace))),
        right_brace: ("}", Ok(Some(Token::RightBrace))),
        left_bracket: ("[", Ok(Some(Token::LeftBracket))),
        right_bracket: ("]", Ok(Some(Token::RightBracket))),
        colon: (":", Ok(Some(Token::Colon))),
        comma: (",", Ok(Some(Token::Comma))),
        null: ("null", Ok(Some(Token::Null))),
        boolean_tru_error: ("tru", Err(LexerError::new("expected 'true', but got tru"))),
        boolean_true: ("true", Ok(Some(Token::Bool(true)))),
        boolean_false: ("false", Ok(Some(Token::Bool(false)))),
        number_integer: ("42", Ok(Some(Token::Number(42f64)))),
        number_float: ("3.14", Ok(Some(Token::Number(3.14)))),
        number_minus: ("-2.85", Ok(Some(Token::Number(-2.85)))),
        number_exp_lowercase: ("2.5e3", Ok(Some(Token::Number(2.5e3)))),
        number_exp_uppercase: ("2.5E3", Ok(Some(Token::Number(2.5e3)))),
        string_basic: (r#""hello, world!""#, Ok(Some(Token::String("hello, world!".to_string())))),
        string_empty: (r#""""#, Ok(Some(Token::String("".to_string())))),
        string_multibytes: (r#""こんにちは！""#, Ok(Some(Token::String("こんにちは！".to_string())))),

        // > If the character is in the Basic
        // > Multilingual Plane (U+0000 through U+FFFF), then it may be
        // > represented as a six-character sequence: a reverse solidus, followed
        // > by the lowercase letter u
        string_unicode: (r#""\u005C""#, Ok(Some(Token::String(r#"\"#.to_string())))),
        string_tab: (r#""\t""#, Ok(Some(Token::String("\t".to_string())))),
        string_escaped_backslash: (r#""\\""#, Ok(Some(Token::String("\\".to_string())))),
    }

    #[test]
    fn tokenize_json() {
        let mut l = Lexer::new(
            r#"{
            "integer": 42,
            "null": null,
            "boolean": false,
            "array": ["v1", "v2"],
            "object": {
                "float": 3.14,
                "exp": -3e10
            }
        }"#,
        );

        let tokens = l.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::LeftBrace,
                Token::String("integer".to_string()),
                Token::Colon,
                Token::Number(42f64),
                Token::Comma,
                Token::String("null".to_string()),
                Token::Colon,
                Token::Null,
                Token::Comma,
                Token::String("boolean".to_string()),
                Token::Colon,
                Token::Bool(false),
                Token::Comma,
                Token::String("array".to_string()),
                Token::Colon,
                Token::LeftBracket,
                Token::String("v1".to_string()),
                Token::Comma,
                Token::String("v2".to_string()),
                Token::RightBracket,
                Token::Comma,
                Token::String("object".to_string()),
                Token::Colon,
                Token::LeftBrace,
                Token::String("float".to_string()),
                Token::Colon,
                Token::Number(3.14),
                Token::Comma,
                Token::String("exp".to_string()),
                Token::Colon,
                Token::Number(-3E10),
                Token::RightBrace,
                Token::RightBrace,
            ]
        )
    }
}
