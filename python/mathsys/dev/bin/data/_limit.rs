//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::reparser::Class;
use crate::runtime::Object;
use crate::Display;
use crate::Debug;


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

//> LIMIT -> IMPLEMENTATION
impl Display for _Limit {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.name())}}
impl Debug for _Limit {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "variable = {}, approach = {}, direction = {}, nest = {}, exponent = {}",
    self.variable, self.approach, self.direction, self.nest, self.exponent
)}} impl Class for _Limit {
    fn name(&self) -> &'static str {"_Limit"}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &Vec<Box<dyn Class>>) -> Object {
        return Object::Undefined(crate::Undefined {});
    }
}