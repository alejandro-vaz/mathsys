//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Context;
use crate::tip::Tip;


//^
//^ NEST
//^

//> NEST -> STRUCT
#[derive(Clone)]
pub struct _Nest {
    pub expression: u32
}

//> NEST -> EVALUATE
impl _Nest {pub fn evaluate(&self, context: &mut Context, id: u32, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let expression = context.get(self.expression, memory);
    //~ EVALUATE -> OPERATIONS
    return Object::Undefined(crate::Undefined {});
}}

//> NEST -> REPRESENTATION
impl crate::Display for _Nest {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Nest {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> NEST -> COMMON
impl Tip for _Nest {} impl _Nest {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Nest")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "expression = {}",
        self.expression
    )}
}