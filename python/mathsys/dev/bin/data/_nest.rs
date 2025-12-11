//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::reparser::Class;
use crate::runtime::Object;
use crate::Display;
use crate::Debug;


//^
//^ NEST
//^

//> NEST -> STRUCT
#[derive(Clone)]
pub struct _Nest {
    pub expression: u32
}

//> NEST -> IMPLEMENTATION
impl Display for _Nest {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.name())}}
impl Debug for _Nest {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "expression = {}",
    self.expression
)}} impl Class for _Nest {
    fn name(&self) -> &'static str {"_Nest"}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &Vec<Box<dyn Class>>) -> Object {
        return Object::Undefined(crate::Undefined {});
    }
}