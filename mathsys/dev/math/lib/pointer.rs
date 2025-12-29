//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    fmt
};


//^
//^ POINTER
//^

//> POINTER -> STRUCT
#[derive(Clone, Copy)]
pub struct Pointer(pub u32);

//> POINTER -> REPRESENTATION
impl fmt::Display for Pointer {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
    "${}",
    self.0
)}}
impl fmt::Debug for Pointer {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
    "${}",
    self.0
)}}

//> POINTER -> BUILD
impl From<u32> for Pointer {fn from(value: u32) -> Pointer {return Pointer(value)}}