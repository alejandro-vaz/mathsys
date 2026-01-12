//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Class, Code, Group, Object, Pointer, Runtime, Tip, Undefined, crash
};


//^
//^ DEFINITION
//^

//> DEFINITION -> STRUCT
#[derive(Clone, Debug)]
pub struct _Definition {
    pub group: Option<Group>,
    pub variable: Pointer,
    pub value: Pointer
}

//> DEFINITION -> EVALUATE
impl _Definition {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let value = runtime.get(self.value, memory);
    let Object::Variable(variable) = runtime.get(self.variable, memory) else {crash(Code::FailedNamedRetrieval)};
    //~ EVALUATE -> OPERATIONS
    self.section("Setting immutable variable", id);
    variable.set(value, false, runtime, self.group.unwrap_or(Group::Undefined));
    return Undefined::new();
}}