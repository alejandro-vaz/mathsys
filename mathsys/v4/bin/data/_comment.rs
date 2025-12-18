//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;
use crate::Display;
use crate::Debug;


//^
//^ COMMENT
//^

//> COMMENT -> STRUCT
#[derive(Clone)]
pub struct _Comment {
    pub text: crate::Box<str>,
}

//> COMMENT -> IMPLEMENTATION
impl Display for _Comment {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.name())}}
impl Debug for _Comment {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "text = \"{}\"",
    self.text
)}} 
impl Class for _Comment {
    fn name(&self) -> &'static str {"_Comment"}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &crate::Vec<crate::Box<dyn Class>>) -> crate::Box<dyn Value> {
        self.space("Comment data", id);
        return crate::Box::new(crate::Nexists {});
    }
}