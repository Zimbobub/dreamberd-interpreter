use crate::runner::variables::VariableType;




pub enum Token {
    Keyword(Keyword),
    Literal(VariableType),
    Whitespace
}

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