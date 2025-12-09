//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::reparser::Class;
use crate::runtime::Value;
use crate::Display;
use crate::Debug;


//^
//^ TENSOR
//^

//> TENSOR -> STRUCT
#[derive(Clone)]
pub struct _Tensor {
    pub values: Box<[u32]>
}

//> TENSOR -> IMPLEMENTATION
impl Display for _Tensor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.name())}}
impl Debug for _Tensor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "values = {:?}",
    self.values
)}} impl Class for _Tensor {
    fn name(&self) -> &'static str {"_Tensor"}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &Vec<Box<dyn Class>>) -> Box<dyn Value> {
        return Box::new(crate::Tensor {})
    }
}