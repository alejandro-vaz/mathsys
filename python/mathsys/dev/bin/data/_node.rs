//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Context;
use crate::tip::Tip;


//^
//^ NODE
//^

//> NODE -> STRUCT
#[derive(Clone)]
pub struct _Node {
    pub expression: u32
}

//> NODE -> EVALUATE
impl _Node {pub fn evaluate(&self, context: &mut Context, id: u32, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let expression = context.get(self.expression, memory);
    //~ EVALUATE -> OPERATIONS
    return Object::Undefined(crate::Undefined {});
}}

//> NODE -> REPRESENTATION
impl crate::Display for _Node {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Node {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> NODE -> COMMON
impl Tip for _Node {} impl _Node {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Node")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "expression = {}",
        self.expression
    )}
}