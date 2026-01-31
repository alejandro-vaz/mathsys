//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;
use crate::Display;
use crate::Debug;


//^
//^ NUMBER
//^

//> NUMBER -> STRUCT
#[derive(Clone)]
pub struct _Number {
    pub value: u32,
    pub shift: u8
}

//> NUMBER -> IMPLEMENTATION
impl Display for _Number {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.name())}}
impl Debug for _Number {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "value = {}, shift = {}",
    self.value, self.shift
)}} impl Class for _Number {
    fn name(&self) -> &'static str {"_Number"}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &crate::Vec<crate::Box<dyn Class>>) -> crate::Box<dyn Value> {
        return crate::Box::new(crate::Number {
            value: self.value,
            shift: self.shift,
            negative: false
        });
    }
}