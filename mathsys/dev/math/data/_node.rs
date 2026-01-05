//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Pointer, Runtime, Class, Object, Tip
};


//^
//^ NODE
//^

//> NODE -> STRUCT
#[derive(Clone, Debug)]
pub struct _Node {
    pub value: Pointer
}

//> NODE -> EVALUATE
impl _Node {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let value = runtime.get(self.value, memory);
    //~ EVALUATE -> OPERATIONS
    self.section("Returning node value", id);
    return value;
}}