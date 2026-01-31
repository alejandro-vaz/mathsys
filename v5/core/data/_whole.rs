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
//^ WHOLE
//^

//> WHOLE -> STRUCT
#[derive(Clone)]
pub struct _Whole {
    pub value: u32
}

//> WHOLE -> EVALUATE
impl _Whole {pub fn evaluate(&self, runtime: &mut Runtime, id: u32, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> OPERATIONS
    self.space("Whole value", id);
    return Object::Whole(crate::Whole::new(
        self.value
    ))
}}

//> WHOLE -> REPRESENTATION
impl crate::Display for _Whole {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Whole {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> WHOLE -> COMMON
impl Tip for _Whole {} impl _Whole {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Whole")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "value = {}",
        self.value
    )}
}