//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ NEST
//^

//> NEST -> STRUCT
pub struct _Nest {
    pub pointer: u32
}

//> NEST -> IMPLEMENTATION
impl Class for _Nest {
    fn name(&self) -> &'static str {"_Nest"}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > pointer = {}",
        self.name(),
        self.pointer
    ))}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32) -> crate::Box<dyn Value> {
        return crate::Box::new(crate::Undefined {});
    }
}