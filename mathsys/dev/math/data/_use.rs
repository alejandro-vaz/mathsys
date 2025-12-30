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
//^ USE
//^

//> USE -> STRUCT
#[derive(Clone)]
pub struct _Use {
    pub name: String,
    pub start: Option<Pointer>
}

//> USE -> EVALUATE
impl _Use {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    if let Some(start) = self.start {runtime.get(start, memory);};
    //~ EVALUATE -> OPERATIONS
    self.section("Use being", id);
    return Undefined::new();
}}

//> USE -> REPRESENTATION
impl fmt::Display for _Use {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for _Use {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> USE -> COMMON
impl Tip for _Use {} impl _Use {
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "_Use")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "name = \"{}\", start = {:?}",
        self.name, self.start
    )}
}