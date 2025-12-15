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
    Nexists = 2,
    Number = 3,
    Tensor = 4,
    Undefined = 0,
    Variable = 5
} 

//> GROUP -> IMPLEMENTATION
impl Group {pub fn from(number: u8) -> Group {return match number {
    1 => Group::Infinite,
    2 => Group::Nexists,
    3 => Group::Number,
    4 => Group::Tensor,
    0 => Group::Undefined,
    5 => Group::Variable,
    other => crash(Code::UnexpectedValue)
}}}