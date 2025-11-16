//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ LIMIT
//^

//> LIMIT -> STRUCT
pub struct Limit {
    variable: u32,
    approach: u32,
    direction: u8,
    pointer: u32,
    exponent: u32
}

//> LIMIT -> IMPLEMENTATION
impl crate::converter::Class for Limit {
    fn name(&self) -> &'static str {"Limit"}
    fn locale(&self, code: u8) -> () {match code {
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn Value> {
        return crate::Box::new(crate::_Undefined {});
    }
} impl Limit {
    pub fn new(variable: u32, approach: u32, direction: u8, pointer: u32, exponent: u32) -> Self {return Limit {
        variable: variable,
        approach: approach,
        direction: direction,
        pointer: pointer,
        exponent: exponent
    }}
}