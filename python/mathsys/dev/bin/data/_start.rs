//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Context;
use crate::tip::Tip;


//^
//^ START
//^

//> START -> STRUCT
#[derive(Clone)]
pub struct _Start {
    pub statements: Box<[u32]>
}

//> START -> EVALUATE
impl _Start {pub fn evaluate(&self, context: &mut Context, id: u32, memory: &Vec<Class>) -> Object {
    for &statement in &self.statements {context.process(statement, memory);}
    return Object::Nexists(crate::Nexists {});
}}

//> START -> REPRESENTATION
impl crate::Display for _Start {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Start {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> START -> COMMON
impl Tip for _Start {} impl _Start {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Start")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "statements = {:?}",
        self.statements
    )}
}