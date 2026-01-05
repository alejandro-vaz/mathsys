//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Class, Group, Object, Pointer, Runtime, Tip
};


//^
//^ CASTS
//^

//> CASTS -> STRUCT
#[derive(Debug, Clone)]
pub struct _Casts {
    pub element: Pointer,
    pub to: Group
}

//> CASTS -> EVALUATE
impl _Casts {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let element = runtime.get(self.element, memory);
    //~ EVALUATE -> OPERATIONS
    self.section("Casting", id);
    return element.cast(self.to);
}}