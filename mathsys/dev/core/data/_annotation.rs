//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Runtime;
use crate::tip::Tip;
use crate::group::Group;
use crate::stdout::{crash, Code};


//^
//^ ANNOTATION
//^

//> ANNOTATION -> STRUCT
#[derive(Clone)]
pub struct _Annotation {
    pub group: u8,
    pub variables: Vec<u32>
}

//> ANNOTATION -> EVALUATE
impl _Annotation {pub fn evaluate(&self, runtime: &mut Runtime, id: u32, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let mut variables = Vec::with_capacity(self.variables.len());
    for &variable in &self.variables {
        let Object::Variable(item) = runtime.get(variable, memory) else {crash(Code::FailedNamedRetrieval)};
        variables.push(item)
    }
    //~ EVALUATE -> OPERATIONS
    self.space("Setting class of variables", id);
    for variable in variables {variable.set(Object::Undefined(crate::Undefined::new()), true, runtime, Group::from(self.group))}
    return Object::Undefined(crate::Undefined::new());
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