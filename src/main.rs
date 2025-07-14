use crate::{lexer::get_tokens, lexer::Token};




mod lexer;
mod parsables;
mod runner;




const SRC: &str = "";


fn main() {
    let tokens: Vec<Token> = get_tokens(SRC);


    dbg!(tokens);
}
