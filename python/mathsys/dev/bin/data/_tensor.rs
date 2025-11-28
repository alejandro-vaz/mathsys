//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;
use crate::Display;
use crate::Debug;


//^
//^ VECTOR
//^

//> VECTOR -> STRUCT
#[derive(Clone)]
pub struct _Tensor {
    pub values: crate::Box<[u32]>
}

//> VECTOR -> IMPLEMENTATION
impl Display for _Tensor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.name())}}
impl Debug for _Tensor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "values = {:?}",
    self.values
)}} impl Class for _Tensor {
    fn name(&self) -> &'static str {"_Tensor"}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &crate::Vec<crate::Box<dyn Class>>) -> crate::Box<dyn Value> {
        return crate::Box::new(crate::Tensor {})
    }
}