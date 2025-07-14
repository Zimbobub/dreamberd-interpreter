use crate::runner::variables::VariableType;



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    file: String,
    line: usize,
    col: usize
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    token_type: TokenType,
    location: Location
}





#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Keyword(Keyword),
    Literal(VariableType),
    Whitespace,
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation)
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