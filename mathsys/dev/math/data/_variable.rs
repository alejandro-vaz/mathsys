//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Pointer,
    Runtime,
    Class,
    Object,
    Variable,
    fmt,
    Tip
};


//^
//^ VARIABLE
//^

//> VARIABLE -> STRUCT
#[derive(Clone)]
pub struct _Variable {
    pub representation: String
}

//> VARIABLE -> EVALUATE
impl _Variable {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> OPERATIONS
    self.section("I am a variable", id);
    return Variable::new(
        self.representation.clone()
    )
}}

//> VARIABLE -> REPRESENTATION
impl fmt::Display for _Variable {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for _Variable {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> VARIABLE -> COMMON
impl Tip for _Variable {} impl _Variable {
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "_Variable")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "representation = \"{}\"",
        self.representation
    )}
}