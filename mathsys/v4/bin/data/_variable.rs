//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;
use crate::Display;
use crate::Debug;


//^
//^ VARIABLE
//^

//> VARIABLE -> STRUCT
#[derive(Clone)]
pub struct _Variable {
    pub representation: crate::Box<str>
}

//> VARIABLE -> IMPLEMENTATION
impl Display for _Variable {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.name())}}
impl Debug for _Variable {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "representation = \"{}\"",
    self.representation
)}} impl Class for _Variable {
    fn name(&self) -> &'static str {"_Variable"}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &crate::Vec<crate::Box<dyn Class>>) -> crate::Box<dyn Value> {
        return crate::Box::new(crate::Variable {
            name: self.representation.clone().into_string()
        });
    }
}