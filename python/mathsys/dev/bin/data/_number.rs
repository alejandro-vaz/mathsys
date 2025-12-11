//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Context;
use crate::tip::Tip;


//^
//^ NUMBER
//^

//> NUMBER -> STRUCT
#[derive(Clone)]
pub struct _Number {
    pub value: u32,
    pub shift: u8
}

//> NUMBER -> EVALUATE
impl _Number {pub fn evaluate(&self, context: &mut Context, id: u32, memory: &Vec<Class>) -> Object {
    return Object::Number(crate::Number {
        value: self.value,
        shift: self.shift,
        negative: false
    })
}}

//> NUMBER -> REPRESENTATION
impl crate::Display for _Number {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Number {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> NUMBER -> COMMON
impl Tip for _Number {} impl _Number {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Number")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "value = {}, shift = {}",
        self.value, self.shift
    )}
}