//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Class, Code, Group, Object, Pointer, Runtime, Tip, Undefined, crash
};


//^
//^ DECLARATION
//^

//> DECLARATION -> STRUCT
#[derive(Clone, Debug)]
pub struct _Declaration {
    pub group: Option<Group>,
    pub variable: Pointer,
    pub value: Pointer
}

//> DECLARATION -> EVALUATE
impl _Declaration {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let value = runtime.get(self.value, memory);
    let Object::Variable(variable) = runtime.get(self.variable, memory) else {crash(Code::FailedNamedRetrieval)};
    //~ EVALUATE -> OPERATIONS
    self.section("Setting mutable variable", id);
    variable.set(value, true, runtime, self.group.unwrap_or(Group::Undefined));
    return Undefined::new();
}}