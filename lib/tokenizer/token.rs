//^
//^ HEAD
//^

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;

//> HEAD -> STRUM_MACROS
use strum_macros::{
    AsRefStr,
    VariantNames
};

//> HEAD -> SUPER
use super::responsibility::Responsibility;


//^
//^ TOKEN
//^

//> TOKEN -> ENUM
#[derive(EnumAsInner, AsRefStr, VariantNames, Debug)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Token<'input> {
    Spaces,
    Identifier {
        name: &'input str
    },
    Module {
        name: &'input str
    },
    Number {
        value: &'input str
    },
    Operator {
        multiplication: bool
    },
    Comment,
    Sign {
        positive: bool
    },
    Definition,
    Close,
    Comma,
    Enter,
    Equality,
    Exit,
    Exponentiation,
    Infinite,
    Limit,
    Newlines,
    Of,
    Open,
    Pipe,
    To,
    Undefined,
    Use,
    EndOfFile
}

//> TOKEN -> IMPLEMENTATION
impl<'valid> Token<'valid> {
    pub fn responsibility(&self) -> Responsibility {return match self {
        Token::Spaces | Token::Comment => Responsibility::Null,
        Token::Module {..} | Token::Sign {..} | Token::Operator {..} | Token::Number {..} | Token::Identifier {..} => Responsibility::Full,
        _ => Responsibility::Structural
    }}
}