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
pub struct Nest {
    pointer: u32
}

//> NEST -> IMPLEMENTATION
impl crate::converter::Class for Nest {
    fn name(&self) -> &'static str {"Nest"}
    fn locale(&self, code: u8) -> () {match code {
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn Value> {
        return crate::Box::new(crate::_Undefined {});
    }
} impl Nest {
    pub fn new(pointer: u32) -> Self {return Nest {
        pointer: pointer
    }}
}