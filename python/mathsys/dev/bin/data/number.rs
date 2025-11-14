//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;


//^
//^ NUMBER
//^

//> NUMBER -> STRUCT
pub struct Number {
    value: u32,
    shift: u8
}

//> NUMBER -> IMPLEMENTATION
impl crate::converter::Class for Number {
    fn name(&self) -> &'static str {"Number"}
    fn locale(&self, code: u8) -> () {match code {
        _ => {crate::stdout::crash(crate::stdout::Code::LocaleNotFound)}
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        return crate::Box::new(crate::_Number {
            value: self.value,
            shift: self.shift
        });
    }
} impl Number {
    pub fn new(value: u32, shift: u8) -> Self {return Number {
        value: value,
        shift: shift
    }}
}