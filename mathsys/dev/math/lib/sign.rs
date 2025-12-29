//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    crash,
    Code,
    fmt,
    Not
};


//^
//^ SIGN
//^

//> SIGN -> STRUCT
#[derive(PartialEq, Clone, Copy)]
pub enum Sign {
    Positive,
    Negative
}

//> SIGN -> FROM U8
impl From<u8> for Sign {fn from(value: u8) -> Self {return match value {
    1 => Sign::Positive,
    2 => Sign::Negative,
    other => crash(Code::UnknownSignValue)
}}}

//> SIGN -> BOOL
impl Into<bool> for Sign {fn into(self) -> bool {return match self {
    Sign::Positive => true,
    Sign::Negative => false
}}}
impl Into<bool> for &Sign {fn into(self) -> bool {return match self {
    Sign::Positive => true,
    Sign::Negative => false
}}}
impl From<bool> for Sign {fn from(value: bool) -> Sign {return if value {Sign::Positive} else {Sign::Negative}}}

//> SIGN -> NOT
impl Not for Sign {type Output = Sign; fn not(self) -> Self::Output {
    return if self.into() {Sign::Negative} else {Sign::Positive}
}}

//> SIGN -> REPRESENTATION
impl fmt::Display for Sign {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
    "{}",
    if self.into() {"+"} else {"-"}
)}}
impl fmt::Debug for Sign {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
    "{}",
    if self.into() {"+"} else {"-"}
)}}