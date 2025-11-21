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
pub struct _Factor {
    pointer: u32,
    expression: u32
}

//> FACTOR -> IMPLEMENTATION
impl Class for _Factor {
    fn name(&self) -> &'static str {"_Factor"}
    fn locale(&self, code: u8) -> () {match code {
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn Value> {
        return crate::Box::new(crate::Undefined {});
    }
} impl _Factor {
    pub fn new(pointer: u32, expression: u32) -> Self {return _Factor {
        pointer: pointer,
        expression: expression
    }}
}