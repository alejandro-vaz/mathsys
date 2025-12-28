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
//^ DECLARATION
//^

//> DECLARATION -> STRUCT
#[derive(Clone)]
pub struct _Declaration {
    pub group: Group,
    pub variable: Pointer,
    pub expression: Pointer
}

//> DECLARATION -> EVALUATE
impl _Declaration {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let expression = runtime.get(self.expression, memory);
    let Object::Variable(variable) = runtime.get(self.variable, memory) else {crash(Code::FailedNamedRetrieval)};
    //~ EVALUATE -> OPERATIONS
    self.section("Setting mutable variable", id);
    variable.set(expression, true, runtime, Group::from(self.group));
    return Undefined::new();
}}

//> DECLARATION -> REPRESENTATION
impl crate::Display for _Declaration {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Declaration {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> DECLARATION -> COMMON
impl Tip for _Declaration {} impl _Declaration {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Declaration")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "group = {}, variable = {}, expression = {}",
        self.group, self.variable, self.expression
    )}
}