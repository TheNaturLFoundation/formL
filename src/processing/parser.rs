use crate::processing::grammar;
use crate::processing::ast::Expr;

pub struct Parser {
    parser_core: grammar::ExprParser,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            parser_core: grammar::ExprParser::new(),
        }
    }
    pub fn parse(&self, expr: &str) -> Box<Expr> {
        return self.parser_core.parse(expr).unwrap();
    }
}