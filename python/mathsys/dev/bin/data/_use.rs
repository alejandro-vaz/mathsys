//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Context;
use crate::tip::Tip;


//^
//^ USE
//^

//> USE -> STRUCT
#[derive(Clone)]
pub struct _Use {
    pub name: Box<str>,
    pub start: u32
}

//> USE -> EVALUATE
impl _Use {pub fn evaluate(&self, context: &mut Context, id: u32, memory: &Vec<Class>) -> Object {
    if self.start != 0 {context.process(self.start, memory)}
    return Object::Nexists(crate::Nexists {});
}}

//> USE -> REPRESENTATION
impl crate::Display for _Use {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Use {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> USE -> COMMON
impl Tip for _Use {} impl _Use {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Use")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "name = \"{}\", start = {}",
        self.name, self.start
    )}
}