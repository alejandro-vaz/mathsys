//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ VARIABLE
//^

//> VARIABLE -> STRUCT
pub struct _Variable {
    pub characters: crate::Box<str>
}

//> VARIABLE -> IMPLEMENTATION
impl Class for _Variable {
    fn name(&self) -> &'static str {"_Variable"}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > characters = \"{}\"",
        self.name(),
        &self.characters
    ))}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32) -> crate::Box<dyn Value> {
        self.space("Processing", id);
        return crate::Box::new(crate::Variable {
            name: self.characters.clone().into_string()
        });
    }
}