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
//^ EQUATION
//^

//> EQUATION -> STRUCT
#[derive(Clone)]
pub struct _Equation {
    pub leftexpression: Pointer,
    pub rightexpression: Pointer
}

//> EQUATION -> EVALUATE
impl _Equation {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let leftexpression = runtime.get(self.leftexpression, memory);
    let rightexpression = runtime.get(self.rightexpression, memory);
    //~ EVALUATE -> OPERATIONS
    self.section("Checking equality", id);
    let equal = leftexpression.equivalency(&rightexpression);
    return Undefined::new();
}}

//> EQUATION -> REPRESENTATION
impl fmt::Display for _Equation {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for _Equation {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> EQUATION -> COMMON
impl Tip for _Equation {} impl _Equation {
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "_Equation")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "leftexpression = {}, rightexpression = {}",
        self.leftexpression, self.rightexpression
    )}
}