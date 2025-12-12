//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Context;
use crate::tip::Tip;


//^
//^ TENSOR
//^

//> TENSOR -> STRUCT
#[derive(Clone)]
pub struct _Tensor {
    pub values: Vec<u32>
}

//> TENSOR -> EVALUATE
impl _Tensor {pub fn evaluate(&self, context: &mut Context, id: u32, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> OPERATIONS
    return Object::Tensor(crate::Tensor {})
}}

//> TENSOR -> REPRESENTATION
impl crate::Display for _Tensor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Tensor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> TENSOR -> COMMON
impl Tip for _Tensor {} impl _Tensor {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Tensor")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "values = {:?}",
        self.values
    )}
}