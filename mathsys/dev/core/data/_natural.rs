//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Runtime;
use crate::tip::Tip;
use crate::group::Group;


//^
//^ NATURAL
//^

//> NATURAL -> STRUCT
#[derive(Clone)]
pub struct _Natural {
    pub value: u32
}

//> NATURAL -> EVALUATE
impl _Natural {pub fn evaluate(&self, runtime: &mut Runtime, id: u32, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> OPERATIONS
    self.space("Natural value", id);
    return Object::Natural(crate::Natural {
        value: self.value
    })
}}

//> NATURAL -> REPRESENTATION
impl crate::Display for _Natural {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Natural {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> NATURAL -> COMMON
impl Tip for _Natural {} impl _Natural {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Natural")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "value = {}",
        self.value
    )}
}