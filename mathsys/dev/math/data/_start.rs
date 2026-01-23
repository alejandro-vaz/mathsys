//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Pointer, Runtime, Class, Object, Undefined, Tip
};


//^
//^ START
//^

//> START -> STRUCT
#[derive(Clone, Debug)]
pub struct _Start {
    pub stream: Vec<Pointer>
}

//> START -> EVALUATE
impl _Start {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //= EVALUATE -> RETRIEVAL
    self.stream.iter().for_each(|statement| {runtime.get(*statement, memory);});
    //= EVALUATE -> OPERATIONS
    self.section("End", id);
    return Undefined::new();
}}