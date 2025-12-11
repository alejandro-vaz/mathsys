//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Context;
use crate::tip::Tip;


//^
//^ FACTOR
//^

//> FACTOR -> STRUCT
#[derive(Clone)]
pub struct _Factor {
    pub value: u32,
    pub exponent: u32
}

//> FACTOR -> EVALUATE
impl _Factor {pub fn evaluate(&self, context: &mut Context, id: u32, memory: &Vec<Class>) -> Object {
    return Object::Undefined(crate::Undefined {});
}}

//> FACTOR -> REPRESENTATION
impl crate::Display for _Factor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Factor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> FACTOR -> COMMON
impl Tip for _Factor {} impl _Factor {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Factor")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "value = {}, exponent = {}",
        self.value, self.exponent
    )}
}