//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Pointer, Runtime, Class, Object, Undefined, Tip
};


//^
//^ USE
//^

//> USE -> STRUCT
#[derive(Clone, Debug)]
pub struct _Use {
    pub name: String,
    pub start: Option<Pointer>
}

//> USE -> EVALUATE
impl _Use {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    if let Some(start) = self.start {runtime.get(start, memory);};
    //~ EVALUATE -> OPERATIONS
    self.section("Use being", id);
    return Undefined::new();
}}