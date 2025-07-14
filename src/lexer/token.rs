use crate::{peekable::Location, runner::variables::VariableType};



#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FileLocation {
    pub file: String,
    pub line: usize,
    pub col: usize
}



impl Location<char> for FileLocation {
    fn advance(&mut self, item: Option<char>) {
        match item {
            Some('\n') => {
                self.col = 0;
                self.line += 1;
            },
            Some(_) => {
                self.col += 1;
            },
            None => {}
        }
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub location: FileLocation
}





#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Keyword(Keyword),
    Literal(VariableType),
    Whitespace,
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),

    Exclamation(u8)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    // Variable related
    Var,
    Const,

    When,

    Previous,
    Next,
    Current,

    Delete,

    // Structure
    If,
    Else,

    Async,
    Function,
    Noop,

    Class,  // can be `class` or `className`

    Reverse,
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperation {
    Not,
    Negate
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    Eq1,
    Eq2,
    Eq3,
    Eq4,

    NotEq1,
    NotEq2,
    NotEq3,
    NotEq4,

    GreaterThan,
    GreaterThanOrEq,

    LessThan,
    LessThanOrEq,
}