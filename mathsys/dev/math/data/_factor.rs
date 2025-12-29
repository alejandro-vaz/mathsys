//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Pointer,
    Runtime,
    Class,
    Object,
    crash,
    Code,
    fmt,
    Tip,
    Group
};


//^
//^ FACTOR
//^

//> FACTOR -> STRUCT
#[derive(Clone)]
pub struct _Factor {
    pub value: Pointer,
    pub exponent: Pointer
}

//> FACTOR -> EVALUATE
impl _Factor {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let mut value = runtime.get(self.value, memory);
    let Object::Nexists(exponent) = runtime.get(self.exponent, memory) else {crash(Code::FailedNamedRetrieval)};
    //~ EVALUATE -> OPERATIONS
    self.section("Computing exponentiation", id);
    if value.is(Group::Variable) {
        let Object::Variable(variable) = value else {crash(Code::FailedNamedRetrieval)};
        value = variable.get(runtime);
    }
    return value;
}}

//> FACTOR -> REPRESENTATION
impl fmt::Display for _Factor {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for _Factor {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> FACTOR -> COMMON
impl Tip for _Factor {} impl _Factor {
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "_Factor")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "value = {}, exponent = {}",
        self.value, self.exponent
    )}
}