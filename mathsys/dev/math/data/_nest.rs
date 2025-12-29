//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Pointer,
    Runtime,
    Class,
    Object,
    fmt,
    Tip
};


//^
//^ NEST
//^

//> NEST -> STRUCT
#[derive(Clone)]
pub struct _Nest {
    pub expression: Pointer
}

//> NEST -> EVALUATE
impl _Nest {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let expression = runtime.get(self.expression, memory);
    //~ EVALUATE -> OPERATIONS
    self.section("Computing nest placeholder", id);
    return expression;
}}

//> NEST -> REPRESENTATION
impl fmt::Display for _Nest {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for _Nest {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> NEST -> COMMON
impl Tip for _Nest {} impl _Nest {
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "_Nest")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "expression = {}",
        self.expression
    )}
}