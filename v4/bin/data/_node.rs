//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;
use crate::Display;
use crate::Debug;


//^
//^ NODE
//^

//> NODE -> STRUCT
#[derive(Clone)]
pub struct _Node {
    pub expression: u32
}

//> NODE -> IMPLEMENTATION
impl Display for _Node {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.name())}}
impl Debug for _Node {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "expression = {}",
    self.expression
)}} impl Class for _Node {
    fn name(&self) -> &'static str {"_Node"}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &crate::Vec<crate::Box<dyn Class>>) -> crate::Box<dyn Value> {
        context.process(self.expression, memory);
        return crate::Box::new(crate::Undefined {});
    }
}