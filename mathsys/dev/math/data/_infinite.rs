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
//^ INFINITE
//^

//> INFINITE -> STRUCT
#[derive(Clone)]
pub struct _Infinite {}

//> INFINITE -> EVALUATE
impl _Infinite {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ INFINITE -> OPERATIONS
    self.section("Getting infinite value", id);
    return Infinite::new(
        true
    )
}}

//> INFINITE -> REPRESENTATION
impl crate::Display for _Infinite {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Infinite {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> INFINITE -> COMMON
impl Tip for _Infinite {} impl _Infinite {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Infinite")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        ""
    )}
}