//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::reparser::Class;
use crate::runtime::Value;
use crate::Display;
use crate::Debug;


//^
//^ START
//^

//> START -> STRUCT
#[derive(Clone)]
pub struct _Start {
    pub statements: Box<[u32]>
}

//> START -> IMPLEMENTATION
impl Display for _Start {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.name())}}
impl Debug for _Start {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "statements = {:?}",
    self.statements
)}} impl Class for _Start {
    fn name(&self) -> &'static str {"_Start"}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &Vec<Box<dyn Class>>) -> Box<dyn Value> {
        for &statement in &self.statements {context.process(statement, memory);}
        return Box::new(crate::Nexists {});
    }
}