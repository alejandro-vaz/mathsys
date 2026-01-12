//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Runtime, Pointer, Class, Object, Whole, Tip, BigUint
};


//^
//^ WHOLE
//^

//> WHOLE -> STRUCT
#[derive(Clone, Debug)]
pub struct _Whole {
    pub value: BigUint
}

//> WHOLE -> EVALUATE
impl _Whole {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> OPERATIONS
    self.section("Whole value", id);
    return Whole::new(
        self.value.clone()
    )
}}