//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ NUMBER
//^

//> NUMBER -> STRUCT
pub struct _Number {
    value: u32,
    shift: u8
}

//> NUMBER -> IMPLEMENTATION
impl Class for _Number {
    fn name(&self) -> &'static str {"_Number"}
    fn locale(&self, code: u8) -> () {match code {
        0 => crate::stdout::debug(&crate::format!(
            "New number is of value {} and shift {}",
            self.value,
            self.shift
        )),
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn Value> {
        self.locale(0);
        return crate::Box::new(crate::Number {
            value: self.value,
            shift: self.shift,
            negative: false
        });
    }
} impl _Number {
    pub fn new(value: u32, shift: u8) -> Self {return _Number {
        value: value,
        shift: shift
    }}
}