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
impl crate::Display for _Equation {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Equation {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> EQUATION -> COMMON
impl Tip for _Equation {} impl _Equation {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Equation")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "leftexpression = {}, rightexpression = {}",
        self.leftexpression, self.rightexpression
    )}
}