pub use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // == lexer internals ==
    #[end]
    ParseEnd,
    #[error]
    ParseError,

    // == keywords ==
    // blocs
    #[token = "do"]
    Do,
    #[token = "end"]
    End,
    // conditionals
    #[token = "if"]
    If,
    #[token = "else"]
    Else,
    #[token = "elsif"]
    Elseif,
    #[token = "unless"]
    Unless,
    #[token = "case"]
    Case,
    #[token = "when"]
    When,
    // loop
    #[token = "loop"]
    Loop,
    #[token = "while"]
    While,
    #[token = "until"]
    Until,
    #[token = "for"]
    For,
    #[token = "break"]
    Break,
    #[token = "next"]
    Next,
    // function
    #[token = "def"]
    Def,

    // == operators ==
    // basic op
    #[token = "+"]
    Plus,
    #[token = "-"]
    Minus,
    #[token = "*"]
    Mul,
    #[token = "/"]
    Div,
    // assignment
    #[token = "="]
    Assignment,
    #[token = "+="]
    PlusAss,
    #[token = "-="]
    MinusAss,
    #[token = "*="]
    MulAss,
    #[token = "/="]
    DivAss,
    // comparisons
    #[token = "=="]
    Eq,
    #[token = "<"]
    LessThan,
    #[token = "<="]
    LessThanOrEq,
    #[token = ">"]
    MoreThan,
    #[token = ">="]
    MoreThanOrEq,
    // priority
    #[token = "("]
    ParOpen,
    #[token = ")"]
    ParClose,

    // == comments ==
    #[regex = "#.*"]
    Comments,

    // == user defined value ==
    #[regex = "[0-9]+"]
    Int,
    #[regex = "[0-9]+\\.[0-9]*"]
    Float,
    #[regex = "\"[^\"]*\""]
    String,
    #[regex = "[a-zA-Z][a-zA-Z0-9_\\-]*"]
    Var,
    #[regex = "\\."]
    Method,
    #[regex = "\n"]
    NewLine,
}
