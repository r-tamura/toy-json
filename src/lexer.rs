use std::fmt::{self, Debug};

type Result<T> = std::result::Result<T, LexerError>;

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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Token::*;
        match self {
            Null => write!(f, "null"),
            Bool(b) => write!(f, "{}", b),
            Number(num) => write!(f, "{}", num),
            String(s) => write!(f, "{}", s),
            LeftBrace => write!(f, "{{"),
            RightBrace => write!(f, "}}"),
            LeftBracket => write!(f, "["),
            RightBracket => write!(f, "]"),
            Colon => write!(f, ":"),
            Comma => write!(f, ","),
        }
    }
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

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            match token {
                Ok(tok) => tokens.push(tok),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(tokens)
    }

    pub fn next_token(&mut self) -> Option<Result<Token>> {
        self.skip_whitespaces();
        if let Some(char) = self.input.peek() {
            match char {
                '{' => Some(self.read_char_and_return_token(Token::LeftBrace)),
                '}' => Some(self.read_char_and_return_token(Token::RightBrace)),
                '[' => Some(self.read_char_and_return_token(Token::LeftBracket)),
                ']' => Some(self.read_char_and_return_token(Token::RightBracket)),
                ':' => Some(self.read_char_and_return_token(Token::Colon)),
                ',' => Some(self.read_char_and_return_token(Token::Comma)),
                'n' => Some(self.read_keyword("null", Token::Null)),
                't' => Some(self.read_keyword("true", Token::Bool(true))),
                'f' => Some(self.read_keyword("false", Token::Bool(false))),
                '-' | '0'..='9' => Some(self.read_number()),
                '"' => Some(self.read_string()),
                _ => Some(Err(LexerError::Common("lexer error".into()))),
            }
        } else {
            None
        }
    }

    fn read_char_and_return_token(&mut self, token: Token) -> Result<Token> {
        assert!(self.input.peek().is_some());
        let _ = self.input.next();
        Ok(token)
    }

    fn read_keyword(&mut self, value: &str, token: Token) -> Result<Token> {
        let cand = self.read_next_chars(value.len());
        if cand == value {
            Ok(token)
        } else {
            Err(LexerError::Common(format!(
                "expected '{}', but got {}",
                value, cand,
            )))
        }
    }

    fn read_number(&mut self) -> Result<Token> {
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
            Ok(Token::Number(n))
        } else {
            Err(LexerError::Common(format!(
                "couldn't parse number '{}'",
                cand
            )))
        }
    }

    fn read_string(&mut self) -> Result<Token> {
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
                    .ok_or(LexerError::UnexpectedEscapedCharacter)?;
                match next_char {
                    // 4HEXDIG ex: \u005C
                    'u' => cand.push(self.read_unicode()?),
                    'b' => cand.push('\u{000B}'), // backspace
                    'f' => cand.push('\u{000C}'), // formfeed
                    'n' => cand.push('\n'),
                    'r' => cand.push('\r'),
                    't' => cand.push('\t'),
                    '\\' => cand.push('\\'),
                    _ => return Err(LexerError::UnexpectedEscapedCharacter),
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
        Ok(Token::String(cand))
    }

    fn read_unicode(&mut self) -> Result<char> {
        let code_point = self.read_next_chars(4);
        let res = u32::from_str_radix(&code_point, 16).map(char::from_u32);
        match res {
            Ok(Some(c)) => Ok(c),
            Ok(None) => Err(LexerError::Common(format!(
                "'{}' is not a code point",
                code_point
            ))),
            Err(err) => return Err(LexerError::Common(format!("{}", err))),
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

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    Common(String),
    Number,
    String,
    Bool,
    Null,
    UnexpectedEscapedCharacter,
    NotCodePoint(char),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::LexerError::*;
        match self {
            Common(msg) => write!(f, "{}", msg),
            UnexpectedEscapedCharacter => write!(f, "unexcepted escaped char"),
            e => write!(f, "{}", e),
        }
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
        empty_string: ("", None),
        white_space: (" ", None),
        left_brace: ("{", Some(Ok(Token::LeftBrace))),
        right_brace: ("}", Some(Ok(Token::RightBrace))),
        left_bracket: ("[", Some(Ok(Token::LeftBracket))),
        right_bracket: ("]", Some(Ok(Token::RightBracket))),
        colon: (":", Some(Ok(Token::Colon))),
        comma: (",", Some(Ok(Token::Comma))),
        null: ("null", Some(Ok(Token::Null))),
        boolean_tru_error: ("tru", Some(Err(LexerError::Common("expected 'true', but got tru".into())))),
        boolean_true: ("true", Some(Ok(Token::Bool(true)))),
        boolean_false: ("false", Some(Ok(Token::Bool(false)))),
        number_integer: ("42", Some(Ok(Token::Number(42f64)))),

        number_float: ("3.14", Some(Ok(Token::Number(#[allow(clippy::approx_constant)]3.14)))),
        number_minus: ("-2.85", Some(Ok(Token::Number(-2.85)))),
        number_exp_lowercase: ("2.5e3", Some(Ok(Token::Number(2.5e3)))),
        number_exp_uppercase: ("2.5E3", Some(Ok(Token::Number(2.5e3)))),
        string_basic: (r#""hello, world!""#, Some(Ok(Token::String("hello, world!".to_string())))),
        string_empty: (r#""""#, Some(Ok(Token::String("".to_string())))),
        string_multibytes: (r#""こんにちは！""#, Some(Ok(Token::String("こんにちは！".to_string())))),

        // > If the character is in the Basic
        // > Multilingual Plane (U+0000 through U+FFFF), then it may be
        // > represented as a six-character sequence: a reverse solidus, followed
        // > by the lowercase letter u
        string_unicode: (r#""\u005C""#, Some(Ok(Token::String(r#"\"#.to_string())))),
        string_tab: (r#""\t""#, Some(Ok(Token::String("\t".to_string())))),
        string_escaped_backslash: (r#""\\""#, Some(Ok(Token::String("\\".to_string())))),
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
                #[allow(clippy::approx_constant)]
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

    #[test]
    fn フォーマットが崩れているとき() {
        let mut l = Lexer::new(
            r#"{"Hello, Wasm!": true, "list"
    : [1,
      2,
  3], "object": {
          "prop1":
    "v1", "prop2": "v2"

}}"#,
        );
        let tokens = l.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LeftBrace,
                Token::String("Hello, Wasm!".to_string()),
                Token::Colon,
                Token::Bool(true),
                Token::Comma,
                Token::String("list".to_string()),
                Token::Colon,
                Token::LeftBracket,
                Token::Number(1f64),
                Token::Comma,
                Token::Number(2f64),
                Token::Comma,
                Token::Number(3f64),
                Token::RightBracket,
                Token::Comma,
                Token::String("object".to_string()),
                Token::Colon,
                Token::LeftBrace,
                Token::String("prop1".to_string()),
                Token::Colon,
                Token::String("v1".to_string()),
                Token::Comma,
                Token::String("prop2".to_string()),
                Token::Colon,
                Token::String("v2".to_string()),
                Token::RightBrace,
                Token::RightBrace,
            ],
        )
    }
}
