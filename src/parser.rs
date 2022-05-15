use std::fmt;

use crate::ast;
use crate::lexer;
use crate::lexer::Token;

type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug)]
pub enum ParserError {
    Lex(lexer::LexerError),
    NoTokenFound,
    UnexpectedToken(Token, Token),
    Syntax(Token),
    Unknown,
}

impl From<lexer::LexerError> for ParserError {
    fn from(err: lexer::LexerError) -> ParserError {
        ParserError::Lex(err)
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParserError::*;
        match self {
            Lex(e) => write!(f, "{}", e),
            NoTokenFound => write!(f, "token is expected, but no token found"),
            UnexpectedToken(actual, expected) => {
                write!(f, "token '{}' is expected, but got '{}'", expected, actual)
            }
            Syntax(_) => write!(f, "Syntax error"),
            _ => write!(f, "unknown error"),
        }
    }
}

pub struct Parser<'a> {
    lex: std::iter::Peekable<&'a mut lexer::Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lex: &'a mut lexer::Lexer<'a>) -> Self {
        Parser {
            lex: lex.peekable(),
        }
    }

    pub fn parse(&mut self) -> Option<Result<ast::Value>> {
        self.lex.next().map(|token| match token {
            Ok(Token::LeftBracket) => self.parse_array(),
            Ok(Token::LeftBrace) => self.parse_object(),
            Ok(Token::Number(r)) => Ok(ast::Value::Number(r)),
            Ok(Token::String(s)) => Ok(ast::Value::String(s)),
            Ok(Token::Bool(false)) => Ok(ast::Value::Bool(false)),
            Ok(Token::Bool(true)) => Ok(ast::Value::Bool(true)),
            Ok(Token::Null) => Ok(ast::Value::Null),
            _ => Err(ParserError::Unknown),
        })
    }

    fn parse_array(&mut self) -> Result<ast::Value> {
        // self.lex.next();
        let mut elements = vec![];
        if self.peek_token_is(Token::RightBracket) {
            return Ok(ast::Value::Array(elements));
        }

        let first = self.parse().unwrap()?;
        elements.push(first);

        while self.peek_token_is(Token::Comma) {
            self.expect_next(); // consume comma
            let e = self.parse().unwrap()?;
            elements.push(e);
        }

        Ok(ast::Value::Array(elements))
    }

    fn parse_object(&mut self) -> Result<ast::Value> {
        let mut pairs = vec![];
        if self.peek_token_is(Token::RightBracket) {
            return Ok(ast::Value::Object(pairs));
        }
        pairs.push(self.parse_key_value()?);

        while self.peek_token_is(Token::Comma) {
            self.expect_next(); // comsume comma
            pairs.push(self.parse_key_value()?);
        }

        Ok(ast::Value::Object(pairs))
    }

    fn parse_key_value(&mut self) -> Result<(String, ast::Value)> {
        let token = self.next_token()?;
        let key = match token {
            Token::String(s) => Ok(s),
            token => Err(ParserError::UnexpectedToken(
                Token::String("string".into()),
                token,
            )),
        }?;

        if !self.peek_token_is(Token::Colon) {
            let actual = self.expect_next();
            return Err(ParserError::UnexpectedToken(Token::Colon, actual));
        }
        self.expect_next();

        let value = self.parse().unwrap()?;

        Ok((key, value))
    }

    fn peek_token_is(&mut self, expected: Token) -> bool {
        let token = self.lex.peek();
        if let Some(token) = token {
            if let Ok(token) = token {
                (*token) == expected
            } else {
                false
            }
        } else {
            false
        }
    }

    fn next_token(&mut self) -> Result<Token> {
        let opts = self.lex.next();
        if opts.is_none() {
            Err(ParserError::NoTokenFound)
        } else {
            opts.unwrap().map_err(|e| ParserError::Lex(e))
        }
    }

    /// 次のトークンが確実に存在する状態で、次のトークンを読み込みます
    /// 次のトークンが存在しない, もしくは, 無効なトークンの場合はpanicを発生させます
    fn expect_next(&mut self) -> Token {
        self.lex
            .next()
            .expect("there should be a token")
            .expect("token should be valid")
    }
}

#[cfg(test)]
mod tests {
    use self::ast::*;
    use super::*;

    #[test]
    fn parse_true() {
        let actual = parse(r#"true"#);
        assert_eq!(actual, ast::Value::Bool(true));
    }

    #[test]
    fn parse_single_null() {
        let actual = parse(r#"null"#);
        assert_eq!(actual, ast::Value::Null);
    }

    #[test]
    fn parse_empty_array() {
        let actual = parse(r#"[]"#);
        assert_eq!(actual, Value::Array(vec![]));
    }

    #[test]
    fn parse_array_with_a_value() {
        let actual = parse(r#"[42]"#);
        assert_eq!(actual, Value::Array(vec![Value::Number(42f64)]));
    }

    #[test]
    fn parse_array_with_values() {
        let actual = parse(r#"["apple", "orange", 42]"#);
        assert_eq!(
            actual,
            Value::Array(vec![
                Value::String("apple".into()),
                Value::String("orange".into()),
                Value::Number(42f64),
            ])
        );
    }

    #[test]
    fn parse_array_nested() {
        let actual = parse(r#"["apple", ["orange", 42, []]]"#);
        assert_eq!(
            actual,
            Value::Array(vec![
                Value::String("apple".into()),
                Value::Array(vec![
                    Value::String("orange".into()),
                    Value::Number(42f64),
                    Value::Array(vec![]),
                ])
            ])
        );
    }

    #[test]
    fn parse_object_with_single_key_value() {
        let actual = parse(r#"{"key": "value"}"#);
        assert_eq!(
            actual,
            Value::Object(vec![("key".into(), Value::String("value".into()))])
        );
    }

    #[test]
    fn parse_object_with_multiple_key_values() {
        let actual = parse(r#"{"name": "Mr.X", "age": 35, "hasCar": true, "education": null}"#);
        assert_eq!(
            actual,
            Value::Object(vec![
                ("name".into(), Value::String("Mr.X".into())),
                ("age".into(), Value::Number(35f64)),
                ("hasCar".into(), Value::Bool(true)),
                ("education".into(), Value::Null),
            ])
        );
    }

    fn parse_object_nested() {
        let actual = parse(r#"{"a": {"b": 2}, "w": {"x": {"y": {"z": null}}}}"#);
        assert_eq!(
            actual,
            Value::Object(vec![
                ("name".into(), Value::String("Mr.X".into())),
                ("age".into(), Value::Number(35f64)),
                ("hasCar".into(), Value::Bool(true)),
                ("education".into(), Value::Null),
            ])
        );
    }

    fn parse<'a>(input: &'a str) -> ast::Value {
        let mut l = lexer::Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let actual = p.parse().unwrap().unwrap();
        actual
    }
}

// https://www.nicovideo.jp/mylist/50993049/?rss=2.0&lang=ja-jp&special_chars_decode=1
