//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Class, Code, Object, Pointer, Runtime, Tip, stdout
};


//^
//^ FACTOR
//^

//> FACTOR -> STRUCT
#[derive(Clone, Debug)]
pub struct _Factor {
    pub value: Pointer,
    pub exponent: Option<Pointer>
}

//> FACTOR -> EVALUATE
impl _Factor {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //= EVALUATE -> RETRIEVAL
    let mut value = runtime.get(self.value, memory);
    if let Object::Variable(variable) = value {value = variable.get(runtime)}
    if let Some(value) = self.exponent {stdout.crash(Code::Todo)}
    //= EVALUATE -> OPERATIONS
    self.section("Computing exponentiation", id);
    return value;
}}