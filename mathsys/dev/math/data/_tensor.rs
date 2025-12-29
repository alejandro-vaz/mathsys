//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Pointer,
    Runtime,
    Class,
    Object,
    Tensor,
    fmt,
    Tip
};


//^
//^ TENSOR
//^

//> TENSOR -> STRUCT
#[derive(Clone)]
pub struct _Tensor {
    pub values: Vec<Pointer>
}

//> TENSOR -> EVALUATE
impl _Tensor {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let mut values = Vec::with_capacity(self.values.len());
    for &value in &self.values {values.push(runtime.get(value, memory))};
    //~ EVALUATE -> OPERATIONS
    self.section("This is a tensor", id);
    return Tensor::new(values);
}}

//> TENSOR -> REPRESENTATION
impl fmt::Display for _Tensor {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for _Tensor {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> TENSOR -> COMMON
impl Tip for _Tensor {} impl _Tensor {
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "_Tensor")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "values = {:?}",
        self.values
    )}
}