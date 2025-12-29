//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Pointer,
    Sign,
    Runtime,
    fmt,
    crash,
    Class,
    Code,
    Object,
    Undefined,
    Tip
};


//^
//^ LIMIT
//^

//> LIMIT -> STRUCT
#[derive(Clone)]
pub struct _Limit {
    pub variable: Pointer,
    pub approach: Pointer,
    pub direction: Sign,
    pub nest: Pointer,
    pub exponent: Pointer
}

//> LIMIT -> EVALUATE
impl _Limit {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let Object::Variable(variable) = runtime.get(self.variable, memory) else {crash(Code::FailedNamedRetrieval)};
    let approach = runtime.get(self.approach, memory);
    //~ EVALUATE -> OPERATIONS
    self.section("Placeholder for limit ops", id);
    return Undefined::new();
}}

//> LIMIT -> REPRESENTATION
impl fmt::Display for _Limit {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for _Limit {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> LIMIT -> COMMON
impl Tip for _Limit {} impl _Limit {
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "_Limit")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "variable = {}, approach = {}, direction = {}, nest = {}, exponent = {}",
        self.variable, self.approach, self.direction, self.nest, self.exponent
    )}
}