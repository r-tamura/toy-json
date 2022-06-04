pub mod ast;
pub mod lexer;
pub mod parser;

pub fn parse(input: &str) -> Option<Result<ast::Value, parser::ParserError>> {
    let mut lex = lexer::Lexer::new(input);
    let mut p = parser::Parser::new(&mut lex);
    p.parse()
}
