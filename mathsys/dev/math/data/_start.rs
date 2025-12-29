//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Pointer,
    Runtime,
    Class,
    Object,
    Undefined,
    fmt,
    Tip
};


//^
//^ START
//^

//> START -> STRUCT
#[derive(Clone)]
pub struct _Start {
    pub statements: Vec<Pointer>
}

//> START -> EVALUATE
impl _Start {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let statements: Vec<Object> = self.statements.iter().map(|&statement| runtime.get(statement, memory)).collect();
    //~ EVALUATE -> OPERATIONS
    self.section("End", id);
    return Undefined::new();
}}

//> START -> REPRESENTATION
impl fmt::Display for _Start {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for _Start {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> START -> COMMON
impl Tip for _Start {} impl _Start {
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "_Start")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "statements = {:?}",
        self.statements
    )}
}