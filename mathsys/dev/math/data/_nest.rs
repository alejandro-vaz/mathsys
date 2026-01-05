//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Pointer, Runtime, Class, Object, Tip, Nexists
};


//^
//^ NEST
//^

//> NEST -> STRUCT
#[derive(Clone, Debug)]
pub struct _Nest {
    pub value: Option<Pointer>
}

//> NEST -> EVALUATE
impl _Nest {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let value = if let Some(value) = self.value {runtime.get(value, memory)} else {Nexists::new()};
    //~ EVALUATE -> OPERATIONS
    self.section("Computing nest placeholder", id);
    return value;
}}