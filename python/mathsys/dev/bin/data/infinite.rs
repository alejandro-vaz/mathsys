//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;


//^
//^ INFINITE
//^

//> INFINITE -> STRUCT
pub struct Infinite {}

//> INFINITE -> IMPLEMENTATION
impl crate::converter::Class for Infinite {
    fn name(&self) -> &'static str {"Infinite"}
    fn locale(&self, code: u8) -> () {match code {
        _ => {crate::stdout::crash(crate::stdout::Code::LocaleNotFound)}
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        return crate::Box::new(crate::_Infinity {
            negative: false
        });
    }
} impl Infinite {
    pub fn new() -> Self {return Infinite {}}
}