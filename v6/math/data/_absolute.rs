//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Runtime, Pointer, Class, Object, Tip
};


//^
//^ ABSOLUTE
//^

//> ABSOLUTE -> STRUCT
#[derive(Clone, Debug)]
pub struct _Absolute {
    pub value: Pointer
}

//> ABSOLUTE -> EVALUATE
impl _Absolute {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let value = runtime.get(self.value, memory);
    //~ EVALUATE -> OPERATIONS
    self.section("Taking absolute value", id);
    return value.absolute();
}}