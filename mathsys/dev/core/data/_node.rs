//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Runtime;
use crate::tip::Tip;
use crate::group::Group;
use crate::stdout::{crash, Code};


//^
//^ NODE
//^

//> NODE -> STRUCT
#[derive(Clone)]
pub struct _Node {
    pub expression: u32
}

//> NODE -> EVALUATE
impl _Node {pub fn evaluate(&self, runtime: &mut Runtime, id: u32, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let expression = runtime.get(self.expression, memory);
    //~ EVALUATE -> OPERATIONS
    self.space("Computing node placeholder", id);
    return Object::Undefined(crate::Undefined::new());
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