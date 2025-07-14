use std::fs;

use crate::{lexer::get_tokens, lexer::Token};



mod peekable;
mod lexer;
mod parsables;
mod runner;




const FILE: &str = "suite/types.db";


fn main() -> Result<(), std::io::Error> {
    // let src = fs::read_to_string(FILE)?;
    let src: String = String::from("!!!!j");

    println!("{}", src.clone());

    let tokens: Vec<Token> = get_tokens(src);

    dbg!(tokens);


    return Ok(());
}
