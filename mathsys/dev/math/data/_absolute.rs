//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Runtime,
    Pointer,
    Class,
    Object,
    fmt,
    Tip
};


//^
//^ ABSOLUTE
//^

//> ABSOLUTE -> STRUCT
#[derive(Clone)]
pub struct _Absolute {
    pub expression: Pointer
}

//> ABSOLUTE -> EVALUATE
impl _Absolute {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let expression = runtime.get(self.expression, memory);
    //~ EVALUATE -> OPERATIONS
    self.section("Taking absolute value", id);
    return expression.absolute();
}}

//> ABSOLUTE -> REPRESENTATION
impl fmt::Display for _Absolute {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for _Absolute {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> ABSOLUTE -> COMMON
impl Tip for _Absolute {} impl _Absolute {
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "_Absolute")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "expression = {}",
        self.expression
    )}
}