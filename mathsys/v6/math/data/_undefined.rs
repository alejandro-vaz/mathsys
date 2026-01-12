//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Pointer, Runtime, Class, Object, Tip, Undefined
};


//^
//^ UNDEFINED
//^

//> UNDEFINED -> CONSTRUCT
#[derive(Clone, Debug)]
pub struct _Undefined {}

//> UNDEFINED -> EVALUATE
impl _Undefined {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> OPERATIONS
    self.section("Getting undefined placeholder", id);
    return Undefined::new();
}}