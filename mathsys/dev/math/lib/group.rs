//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    fmt,
    crash,
    Code
};


//^
//^ GROUP
//^

//> GROUP -> STRUCT
#[derive(PartialEq, Copy, Clone)]
pub enum Group {
    Infinite = 1,
    Integer = 2,
    Natural = 3,
    Nexists = 4,
    Rational = 5,
    Tensor = 6,
    Undefined = 0,
    Variable = 7,
    Whole = 8
} 

//> GROUP -> FROM
impl From<u8> for Group {fn from(number: u8) -> Group {return match number {
    1 => Group::Infinite,
    2 => Group::Integer,
    3 => Group::Natural,
    4 => Group::Nexists,
    5 => Group::Rational,
    6 => Group::Tensor,
    0 => Group::Undefined,
    7 => Group::Variable,
    8 => Group::Whole,
    other => crash(Code::UnknownGroupCode)
}}}

//> GROUP -> DISPLAY
impl fmt::Display for Group {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "{}", match self {
    Group::Infinite => "Infinite",
    Group::Integer => "Integer",
    Group::Natural => "Natural",
    Group::Nexists => "Nexists",
    Group::Rational => "Rational",
    Group::Tensor => "Tensor",
    Group::Undefined => "Undefined",
    Group::Variable => "Variable",
    Group::Whole => "Whole"
})}}