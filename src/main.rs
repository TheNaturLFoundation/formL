mod processing;
//mod session;

use processing::parser::Parser;

fn main() {
    let parser = Parser::new();
    let expr = parser.parse("(1 + 5) * 2").evaluate();
    dbg!(expr);
}