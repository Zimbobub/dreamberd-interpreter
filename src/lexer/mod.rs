
mod token;
pub use token::Token;

use crate::{lexer::token::{FileLocation, TokenType}, peekable::Peekable};


pub fn get_tokens(src: String) -> Vec<Token> {
    let mut char_stream: Peekable<char, FileLocation> = Peekable::from_string(src);
    let mut tokens: Vec<Token> = Vec::new();
    
    dbg!(char_stream.is_empty());


    while !char_stream.is_empty() {
        println!("{}", char_stream.peek().unwrap());
        if char_stream.next_char_is('j') { break; }

        // exclamations
        if char_stream.next_char_is('!') {

            let mut counter = 0;
            while char_stream.next_char_is('!') {
                char_stream.consume(1);
                counter += 1;
            }
            tokens.push(Token {
                token_type: TokenType::Exclamation(counter),
                location: char_stream.location.clone()
            });
        }


    };
    
    return tokens;
}