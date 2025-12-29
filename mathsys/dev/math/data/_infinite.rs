//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Runtime,
    Infinite,
    Pointer,
    Class,
    Object,
    fmt,
    Tip
};


//^
//^ INFINITE
//^

//> INFINITE -> STRUCT
#[derive(Clone)]
pub struct _Infinite {}

//> INFINITE -> EVALUATE
impl _Infinite {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ INFINITE -> OPERATIONS
    self.section("Getting infinite value", id);
    return Infinite::new(
        true
    )
}}

//> INFINITE -> REPRESENTATION
impl fmt::Display for _Infinite {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for _Infinite {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> INFINITE -> COMMON
impl Tip for _Infinite {} impl _Infinite {
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "_Infinite")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        ""
    )}
}