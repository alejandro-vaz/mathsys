//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Pointer, Runtime, Class, Object, Variable, Tip
};


//^
//^ VARIABLE
//^

//> VARIABLE -> STRUCT
#[derive(Clone, Debug)]
pub struct _Variable {
    pub representation: String
}

//> VARIABLE -> EVALUATE
impl _Variable {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> OPERATIONS
    self.section("I am a variable", id);
    return Variable::new(
        self.representation.clone()
    )
}}