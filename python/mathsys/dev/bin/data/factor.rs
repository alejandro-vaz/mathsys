//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ FACTOR
//^

//> FACTOR -> STRUCT
pub struct Factor {
    pointer: u32,
    expression: u32
}

//> FACTOR -> IMPLEMENTATION
impl crate::converter::Class for Factor {
    fn name(&self) -> &'static str {"Factor"}
    fn locale(&self, code: u8) -> () {match code {
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn Value> {
        return crate::Box::new(crate::_Undefined {});
    }
} impl Factor {
    pub fn new(pointer: u32, expression: u32) -> Self {return Factor {
        pointer: pointer,
        expression: expression
    }}
}