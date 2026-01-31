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
impl _Limit {pub fn evaluate(&self, runtime: &mut Runtime, id: u32, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let Object::Variable(variable) = runtime.get(self.variable, memory) else {crash(Code::FailedNamedRetrieval)};
    let approach = runtime.get(self.variable, memory);
    //~ EVALUATE -> OPERATIONS
    self.space("Placeholder for limit ops", id);
    return Object::Undefined(crate::Undefined::new());
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