mod lex;
mod parse;

use crate::lex::{Lexer, TokenType};
fn main() {
    let source = "IF+-123 foo*THEN/";
    let mut lex = Lexer::new(source.to_string());

    let mut token = lex.get_token();

    while token.kind != TokenType::EOF {
        println!("{:?}", token.kind);
        token = lex.get_token();
    }
}
