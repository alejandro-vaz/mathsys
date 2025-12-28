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
//^ ANNOTATION
//^

//> ANNOTATION -> STRUCT
#[derive(Clone)]
pub struct _Annotation {
    pub group: Group,
    pub variables: Vec<Pointer>
}

//> ANNOTATION -> EVALUATE
impl _Annotation {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let mut variables = Vec::with_capacity(self.variables.len());
    for &variable in &self.variables {
        let Object::Variable(item) = runtime.get(variable, memory) else {crash(Code::FailedNamedRetrieval)};
        variables.push(item)
    }
    //~ EVALUATE -> OPERATIONS
    self.section("Setting class of variables", id);
    for variable in variables {variable.set(Undefined::new(), true, runtime, self.group)}
    return Undefined::new();
}}

//> ANNOTATION -> REPRESENTATION
impl crate::Display for _Annotation {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Annotation {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> ANNOTATION -> COMMON
impl Tip for _Annotation {} impl _Annotation {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Annotation")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "group = {}, variables = {:?}",
        self.group, self.variables
    )}
}