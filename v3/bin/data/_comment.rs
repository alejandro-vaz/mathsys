//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ COMMENT
//^

//> COMMENT -> STRUCT
pub struct _Comment {
    pub characters: crate::Box<str>,
}

//> COMMENT -> IMPLEMENTATION
impl Class for _Comment {
    fn name(&self) -> &'static str {"_Comment"}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > characters = \"{}\"",
        self.name(),
        &self.characters
    ))}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32) -> crate::Box<dyn Value> {
        return crate::Box::new(crate::Nexists {});
    }
}