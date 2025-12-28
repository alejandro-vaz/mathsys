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
//^ DEFINITION
//^

//> DEFINITION -> STRUCT
#[derive(Clone)]
pub struct _Definition {
    pub group: Group,
    pub variable: Pointer,
    pub expression: Pointer
}

//> DEFINITION -> EVALUATE
impl _Definition {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let expression = runtime.get(self.expression, memory);
    let Object::Variable(variable) = runtime.get(self.variable, memory) else {crash(Code::FailedNamedRetrieval)};
    //~ EVALUATE -> OPERATIONS
    self.section("Setting immutable variable", id);
    variable.set(expression, false, runtime, Group::from(self.group));
    return Undefined::new();
}}

//> DEFINITION -> REPRESENTATION
impl crate::Display for _Definition {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Definition {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> DEFINITION -> COMMON
impl Tip for _Definition {} impl _Definition {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Definition")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "group = {}, variable = {}, expression = {}",
        self.group, self.variable, self.expression
    )}
}