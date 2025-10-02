use crate::lexer::Lexer;

pub mod token;
pub mod lexer;

fn main() {
    let src = include_str!("script.qte").to_string();
    let mut lexer = Lexer::new(src);

    let tokens = lexer.tokenize();

    dbg!(&tokens);
}
