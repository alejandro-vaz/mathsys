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
//^ ABSOLUTE
//^

//> ABSOLUTE -> STRUCT
#[derive(Clone)]
pub struct _Absolute {
    pub expression: Pointer
}

//> ABSOLUTE -> EVALUATE
impl _Absolute {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let expression = runtime.get(self.expression, memory);
    //~ EVALUATE -> OPERATIONS
    self.section("Taking absolute value", id);
    return expression.absolute();
}}

//> ABSOLUTE -> REPRESENTATION
impl crate::Display for _Absolute {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Absolute {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> ABSOLUTE -> COMMON
impl Tip for _Absolute {} impl _Absolute {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Absolute")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "expression = {}",
        self.expression
    )}
}