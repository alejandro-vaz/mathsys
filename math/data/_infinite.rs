//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Runtime, Infinite, Pointer, Class, Object, Tip, Sign
};


//^
//^ INFINITE
//^

//> INFINITE -> STRUCT
#[derive(Clone, Debug)]
pub struct _Infinite {}

//> INFINITE -> EVALUATE
impl _Infinite {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //= INFINITE -> OPERATIONS
    self.section("Getting infinite value", id);
    return Infinite::new(
        Sign::Positive
    );
}}