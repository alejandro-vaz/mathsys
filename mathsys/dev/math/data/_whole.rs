//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Runtime,
    Pointer,
    Class,
    Object,
    Whole,
    fmt,
    Tip,
    BigUint
};


//^
//^ WHOLE
//^

//> WHOLE -> STRUCT
#[derive(Clone)]
pub struct _Whole {
    pub value: BigUint
}

//> WHOLE -> EVALUATE
impl _Whole {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> OPERATIONS
    self.section("Whole value", id);
    return Whole::new(
        self.value.clone()
    )
}}

//> WHOLE -> REPRESENTATION
impl fmt::Display for _Whole {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for _Whole {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> WHOLE -> COMMON
impl Tip for _Whole {} impl _Whole {
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "_Whole")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "value = {}",
        self.value
    )}
}