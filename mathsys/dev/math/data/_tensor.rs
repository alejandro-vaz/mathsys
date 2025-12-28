//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::{Infinite, Integer, Natural, Nexists, Rational, Tensor, Undefined, Variable, Whole};
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Runtime;
use crate::tip::Tip;
use crate::group::Group;
use crate::stdout::{crash, Code};
use crate::types::Pointer;


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
    //~ EVALUATE -> OPERATIONS
    self.section("This is a tensor", id);
    return Tensor::new();
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