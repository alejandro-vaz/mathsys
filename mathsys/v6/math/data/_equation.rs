//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Pointer, Runtime, Class, Object, Undefined, Tip
};


//^
//^ EQUATION
//^

//> EQUATION -> STRUCT
#[derive(Clone, Debug)]
pub struct _Equation {
    pub leftside: Pointer,
    pub rightside: Pointer
}

//> EQUATION -> EVALUATE
impl _Equation {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let leftside = runtime.get(self.leftside, memory);
    let rightside = runtime.get(self.rightside, memory);
    //~ EVALUATE -> OPERATIONS
    self.section("Checking equality", id);
    let equal = leftside.equivalency(&rightside);
    return Undefined::new();
}}