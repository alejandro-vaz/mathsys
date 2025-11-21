//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ VECTOR
//^

//> VECTOR -> STRUCT
pub struct _Tensor {
    values: crate::Box<[u32]>
}

//> VECTOR -> IMPLEMENTATION
impl Class for _Tensor {
    fn name(&self) -> &'static str {"_Tensor"}
    fn locale(&self, code: u8) -> () {match code {
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn Value> {
        return crate::Box::new(crate::Tensor {})
    }
} impl _Tensor {
    pub fn new(values: &[u32]) -> Self {return _Tensor {
        values: values.into()
    }}
}