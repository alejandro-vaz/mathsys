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
//^ FACTOR
//^

//> FACTOR -> STRUCT
#[derive(Clone)]
pub struct _Factor {
    pub value: Pointer,
    pub exponent: Option<Pointer>
}

//> FACTOR -> EVALUATE
impl _Factor {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let mut value = runtime.get(self.value, memory);
    if let Object::Variable(variable) = value {value = variable.get(runtime)}
    //~ EVALUATE -> OPERATIONS
    self.section("Computing exponentiation", id);
    return value;
}}

//> FACTOR -> REPRESENTATION
impl fmt::Display for _Factor {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for _Factor {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> FACTOR -> COMMON
impl Tip for _Factor {} impl _Factor {
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "_Factor")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "value = {}, exponent = {:?}",
        self.value, self.exponent
    )}
}