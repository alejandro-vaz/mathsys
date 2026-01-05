//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    fmt, crash, Code, AsRefStr
};


//^
//^ GROUP
//^

//> GROUP -> STRUCT
#[derive(PartialEq, Copy, Clone, AsRefStr)]
pub enum Group {
    Infinite,
    Integer,
    Natural,
    Nexists,
    Rational,
    Tensor,
    Undefined,
    Variable,
    Whole
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

//> GROUP -> DEBUG
impl fmt::Debug for Group {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
    "@{}",
    self.as_ref()
)}}