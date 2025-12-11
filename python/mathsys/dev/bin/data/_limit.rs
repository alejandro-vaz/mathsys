//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Context;
use crate::tip::Tip;


//^
//^ LIMIT
//^

//> LIMIT -> STRUCT
#[derive(Clone)]
pub struct _Limit {
    pub variable: u32,
    pub approach: u32,
    pub direction: u8,
    pub nest: u32,
    pub exponent: u32
}

//> LIMIT -> EVALUATE
impl _Limit {pub fn evaluate(&self, context: &mut Context, id: u32, memory: &Vec<Class>) -> Object {
    return Object::Undefined(crate::Undefined {});
}}

//> LIMIT -> REPRESENTATION
impl crate::Display for _Limit {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Limit {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> LIMIT -> COMMON
impl Tip for _Limit {} impl _Limit {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Limit")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "variable = {}, approach = {}, direction = {}, nest = {}, exponent = {}",
        self.variable, self.approach, self.direction, self.nest, self.exponent
    )}
}