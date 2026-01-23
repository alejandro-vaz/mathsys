//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Pointer, Sign, Runtime, stdout, Class, Code, Object, Undefined, Tip
};


//^
//^ LIMIT
//^

//> LIMIT -> STRUCT
#[derive(Clone, Debug)]
pub struct _Limit {
    pub variable: Pointer,
    pub approach: Pointer,
    pub direction: Option<Sign>,
    pub nest: Pointer,
    pub exponent: Option<Pointer>
}

//> LIMIT -> EVALUATE
impl _Limit {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //= EVALUATE -> RETRIEVAL
    let Object::Variable(variable) = runtime.get(self.variable, memory) else {stdout.crash(Code::FailedNamedRetrieval)};
    //= EVALUATE -> OPERATIONS
    self.section("Placeholder for limit ops", id);
    stdout.crash(Code::Todo);
    return Undefined::new();
}}