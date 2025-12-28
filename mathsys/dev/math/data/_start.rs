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
//^ START
//^

//> START -> STRUCT
#[derive(Clone)]
pub struct _Start {
    pub statements: Vec<Pointer>
}

//> START -> EVALUATE
impl _Start {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let mut statements = Vec::with_capacity(self.statements.len());
    for &statement in &self.statements {statements.push(runtime.get(statement, memory))}
    //~ EVALUATE -> OPERATIONS
    self.section("End", id);
    return Undefined::new();
}}

//> START -> REPRESENTATION
impl crate::Display for _Start {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Start {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> START -> COMMON
impl Tip for _Start {} impl _Start {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Start")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "statements = {:?}",
        self.statements
    )}
}