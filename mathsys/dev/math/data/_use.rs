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
//^ USE
//^

//> USE -> STRUCT
#[derive(Clone)]
pub struct _Use {
    pub name: String,
    pub start: Pointer
}

//> USE -> EVALUATE
impl _Use {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let start = runtime.get(self.start, memory);
    //~ EVALUATE -> OPERATIONS
    self.section("Use being", id);
    return Undefined::new();
}}

//> USE -> REPRESENTATION
impl crate::Display for _Use {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Use {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> USE -> COMMON
impl Tip for _Use {} impl _Use {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Use")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "name = \"{}\", start = {}",
        self.name, self.start
    )}
}