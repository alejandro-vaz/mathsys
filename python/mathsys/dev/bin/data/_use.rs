//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;
use crate::Display;
use crate::Debug;


//^
//^ USE
//^

//> USE -> STRUCT
#[derive(Clone)]
pub struct _Use {
    pub name: crate::Box<str>,
    pub start: u32
}

//> USE -> IMPLEMENTATION
impl Display for _Use {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.name())}}
impl Debug for _Use {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "name = \"{}\", start = {}",
    self.name, self.start
)}} impl Class for _Use {
    fn name(&self) -> &'static str {"_Use"}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &crate::Vec<crate::Box<dyn Class>>) -> crate::Box<dyn Value> {
        if self.start != 0 {context.process(self.start, memory)}
        return crate::Box::new(crate::Nexists {});
    }
}