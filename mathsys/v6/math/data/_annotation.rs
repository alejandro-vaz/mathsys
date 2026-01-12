//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Class, Code, Group, Object, Pointer, Runtime, Tip, Undefined, Variable, crash
};


//^
//^ ANNOTATION
//^

//> ANNOTATION -> STRUCT
#[derive(Clone, Debug)]
pub struct _Annotation {
    pub group: Group,
    pub variables: Vec<Pointer>
}

//> ANNOTATION -> EVALUATE
impl _Annotation {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let variables = self.variables.iter().map(|variable| {
        let Object::Variable(item) = runtime.get(*variable, memory) else {crash(Code::FailedNamedRetrieval)};
        item
    }).collect::<Vec<Variable>>();
    //~ EVALUATE -> OPERATIONS
    self.section("Setting class of variables", id);
    variables.iter().for_each(|variable| variable.set(Undefined::new(), true, runtime, self.group));
    return Undefined::new();
}}