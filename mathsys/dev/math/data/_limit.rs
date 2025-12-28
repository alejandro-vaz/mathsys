//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::{Infinite, Integer, Natural, Nexists, Rational, Tensor, Undefined, Variable, Whole};
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Runtime;
use crate::tip::Tip;
use crate::group::Group;
use crate::stdout::{crash, Code};
use crate::types::Pointer;


//^
//^ LIMIT
//^

//> LIMIT -> STRUCT
#[derive(Clone)]
pub struct _Limit {
    pub variable: Pointer,
    pub approach: Pointer,
    pub direction: u8,
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
impl crate::Display for _Limit {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Limit {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> LIMIT -> COMMON
impl Tip for _Limit {} impl _Limit {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Limit")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "variable = {}, approach = {}, direction = {}, nest = {}, exponent = {}",
        self.variable, self.approach, self.direction, self.nest, self.exponent
    )}
}