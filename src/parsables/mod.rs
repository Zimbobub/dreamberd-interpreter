use crate::token::Token;




pub mod program;
pub mod block;
pub mod statement;
pub mod expr;



pub trait Parsable {
    fn parse(tokens: Vec<Token>) -> Self;
}