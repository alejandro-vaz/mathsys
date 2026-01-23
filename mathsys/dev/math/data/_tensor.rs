//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Pointer, Runtime, Class, Object, Tensor, Tip
};


//^
//^ TENSOR
//^

//> TENSOR -> STRUCT
#[derive(Clone, Debug)]
pub struct _Tensor {
    pub values: Vec<Pointer>
}

//> TENSOR -> EVALUATE
impl _Tensor {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //= EVALUATE -> RETRIEVAL
    let values = self.values.iter().map(|value| runtime.get(*value, memory)).collect();
    //= EVALUATE -> OPERATIONS
    self.section("This is a tensor", id);
    return Tensor::new(values);
}}