//^
//^ HEAD
//^

//> HEAD -> STRUM_MACROS
use strum_macros::{
    EnumCount,
    EnumString
};


//^
//^ TOKEN
//^

//> TOKEN -> KIND
#[derive(Clone, Copy, EnumCount, EnumString, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Kind {
    IDENTIFIER,
    MODULE,
    NUMBER,
    OPERATOR,
    COMMENT,
    RATIONAL,
    SIGN,
    DEFINITION,
    CLOSE,
    COMMA,
    ENTER,
    EQUALITY,
    EXIT,
    EXPONENTIATION,
    INFINITE,
    LIMIT,
    NEWLINES,
    OF,
    OPEN,
    PIPE,
    SPACES,
    TO,
    UNDEFINED,
    USE,
    ENDOFFILE
}

//> TOKEN -> STRUCT
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Token<'input> {
    pub value: &'input str,
    pub kind: Kind
} impl<'input> Token<'input> {
    pub fn new(cursor: usize, length: usize, kind: Kind, string: &'input str) -> Self {return Self {
        value: &string[cursor .. cursor + length],
        kind: kind
    }}
}