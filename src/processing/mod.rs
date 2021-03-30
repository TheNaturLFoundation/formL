use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar, "/processing/grammar.rs");
pub mod ast;
pub mod parser;