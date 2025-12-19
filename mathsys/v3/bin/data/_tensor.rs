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
    pub values: crate::Box<[u32]>
}

//> VECTOR -> IMPLEMENTATION
impl Class for _Tensor {
    fn name(&self) -> &'static str {"_Tensor"}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > values = [{}]",
        self.name(),
        self.values.iter().map(|id| crate::format!("{}", id)).collect::<crate::Vec<_>>().join(", ")
    ))}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32) -> crate::Box<dyn Value> {
        return crate::Box::new(crate::Tensor {})
    }
}