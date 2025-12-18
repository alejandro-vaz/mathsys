//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::stdout::{crash, Code};


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
    Tensor = 5,
    Undefined = 0,
    Variable = 6,
    Whole = 7
} 

//> GROUP -> IMPLEMENTATION
impl Group {pub fn from(number: u8) -> Group {return match number {
    1 => Group::Infinite,
    2 => Group::Integer,
    3 => Group::Natural,
    4 => Group::Nexists,
    5 => Group::Tensor,
    0 => Group::Undefined,
    6 => Group::Variable,
    7 => Group::Whole,
    other => crash(Code::UnknownGroupCode)
}}}