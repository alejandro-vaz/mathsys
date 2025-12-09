//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::reparser::Class;
use crate::runtime::Value;
use crate::Display;
use crate::Debug;


//^
//^ FACTOR
//^

//> FACTOR -> STRUCT
#[derive(Clone)]
pub struct _Factor {
    pub value: u32,
    pub exponent: u32
}

//> FACTOR -> IMPLEMENTATION
impl Display for _Factor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.name())}}
impl Debug for _Factor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "value = {}, exponent = {}",
    self.value, self.exponent
)}} impl Class for _Factor {
    fn name(&self) -> &'static str {"_Factor"}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &Vec<Box<dyn Class>>) -> Box<dyn Value> {
        return Box::new(crate::Undefined {});
    }
}