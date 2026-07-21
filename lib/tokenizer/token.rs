//^
//^ HEAD
//^

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;

//> HEAD -> STRUM_MACROS
use strum_macros::AsRefStr;

//> HEAD -> SUPER
use super::responsibility::Responsibility;


//^
//^ TOKEN
//^

//> TOKEN -> ENUM
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, EnumAsInner, AsRefStr, PartialOrd, Ord)]
pub enum Token<'input> {
    SPACES,
    IDENTIFIER {
        name: &'input str
    },
    MODULE {
        name: &'input str
    },
    NUMBER {
        value: &'input str
    },
    OPERATOR {
        multiplication: bool
    },
    COMMENT,
    SIGN {
        positive: bool
    },
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
    TO,
    UNDEFINED,
    USE,
    ENDOFFILE
}

//> TOKEN -> IMPLEMENTATION
impl<'valid> Token<'valid> {
    pub fn responsibility(self) -> Responsibility {return match self {
        Token::SPACES | Token::COMMENT => Responsibility::Null,
        Token::MODULE {..} | Token::SIGN {..} | Token::OPERATOR {..} | Token::NUMBER {..} | Token::IDENTIFIER {..} => Responsibility::Full,
        _ => Responsibility::Structural
    }}
}