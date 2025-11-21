//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ INFINITE
//^

//> INFINITE -> STRUCT
pub struct _Infinite {}

//> INFINITE -> IMPLEMENTATION
impl Class for _Infinite {
    fn name(&self) -> &'static str {"_Infinite"}
    fn locale(&self, code: u8) -> () {match code {
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn Value> {
        return crate::Box::new(crate::Infinite {
            negative: false
        });
    }
} impl _Infinite {
    pub fn new() -> Self {return _Infinite {}}
}