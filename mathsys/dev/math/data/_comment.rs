//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Runtime,
    Pointer,
    Class,
    Object,
    Undefined,
    fmt,
    Tip
};


//^
//^ COMMENT
//^

//> COMMENT -> STRUCT
#[derive(Clone)]
pub struct _Comment {
    pub text: String
}

//> COMMENT -> EVALUATE
impl _Comment {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> OPERATIONS
    self.section("Comment data", id);
    return Undefined::new();
}}

//> COMMENT -> REPRESENTATION
impl fmt::Display for _Comment {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for _Comment {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> COMMENT -> COMMON
impl Tip for _Comment {} impl _Comment {
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "_Comment")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "text = \"{}\"",
        self.text
    )}
}