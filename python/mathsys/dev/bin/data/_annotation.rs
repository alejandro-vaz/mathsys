//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ ANNOTATION
//^

//> ANNOTATION -> STRUCT
pub struct _Annotation {
    object: u8,
    variable: u32
}

//> ANNOTATION -> IMPLEMENTATION
impl Class for _Annotation {
    fn name(&self) -> &'static str {"_Annotation"}
    fn locale(&self, code: u8) -> () {match code {
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn Value> {
        context.process(self.variable);
        return crate::Box::new(crate::Nexists {});
    }
} impl _Annotation {
    pub fn new(object: u8, variable: u32) -> Self {return _Annotation {
        object: object,
        variable: variable
    }}
}