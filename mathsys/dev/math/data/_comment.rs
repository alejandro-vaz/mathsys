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
//^ COMMENT
//^

//> COMMENT -> STRUCT
#[derive(Clone)]
pub struct _Comment {
    pub text: String
}

//> COMMENT -> EVALUATE
impl _Comment {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> OPERATIONS
    self.section("Comment data", id);
    return Undefined::new();
}}

//> COMMENT -> REPRESENTATION
impl crate::Display for _Comment {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Comment {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> COMMENT -> COMMON
impl Tip for _Comment {} impl _Comment {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Comment")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "text = \"{}\"",
        self.text
    )}
}