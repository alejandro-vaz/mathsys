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
pub struct Vector {
    values: crate::Box<[u32]>
}

//> VECTOR -> IMPLEMENTATION
impl crate::converter::Class for Vector {
    fn name(&self) -> &'static str {"Vector"}
    fn locale(&self, code: u8) -> () {match code {
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn Value> {
        return crate::Box::new(crate::_Undefined {})
    }
} impl Vector {
    pub fn new(values: &[u32]) -> Self {return Vector {
        values: values.into()
    }}
}