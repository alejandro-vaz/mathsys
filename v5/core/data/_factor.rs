//^
//^ HEAD
//^

use std::env::var;

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Runtime;
use crate::tip::Tip;
use crate::group::Group;
use crate::stdout::{crash, Code};


//^
//^ FACTOR
//^

//> FACTOR -> STRUCT
#[derive(Clone)]
pub struct _Factor {
    pub value: u32,
    pub exponent: u32
}

//> FACTOR -> EVALUATE
impl _Factor {pub fn evaluate(&self, runtime: &mut Runtime, id: u32, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let mut value = runtime.get(self.value, memory);
    let Object::Nexists(exponent) = runtime.get(self.exponent, memory) else {crash(Code::FailedNamedRetrieval)};
    //~ EVALUATE -> OPERATIONS
    self.space("Computing exponentiation", id);
    if value.is(Group::Variable) {
        let Object::Variable(variable) = value else {crash(Code::FailedNamedRetrieval)};
        value = variable.get(runtime);
    }
    return value;
}}

//> FACTOR -> REPRESENTATION
impl crate::Display for _Factor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Factor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> FACTOR -> COMMON
impl Tip for _Factor {} impl _Factor {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Factor")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "value = {}, exponent = {}",
        self.value, self.exponent
    )}
}