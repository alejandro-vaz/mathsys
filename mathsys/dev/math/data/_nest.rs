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
//^ NEST
//^

//> NEST -> STRUCT
#[derive(Clone)]
pub struct _Nest {
    pub expression: Pointer
}

//> NEST -> EVALUATE
impl _Nest {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let expression = runtime.get(self.expression, memory);
    //~ EVALUATE -> OPERATIONS
    self.section("Computing nest placeholder", id);
    return expression;
}}

//> NEST -> REPRESENTATION
impl crate::Display for _Nest {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Nest {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> NEST -> COMMON
impl Tip for _Nest {} impl _Nest {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Nest")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "expression = {}",
        self.expression
    )}
}