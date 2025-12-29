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
//^ NODE
//^

//> NODE -> STRUCT
#[derive(Clone)]
pub struct _Node {
    pub expression: Pointer
}

//> NODE -> EVALUATE
impl _Node {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let expression = runtime.get(self.expression, memory);
    //~ EVALUATE -> OPERATIONS
    self.section("Returning node value", id);
    return expression;
}}

//> NODE -> REPRESENTATION
impl fmt::Display for _Node {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for _Node {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> NODE -> COMMON
impl Tip for _Node {} impl _Node {
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "_Node")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "expression = {}",
        self.expression
    )}
}