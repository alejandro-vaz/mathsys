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
pub struct Variable {
    characters: crate::Box<str>
}

//> VARIABLE -> IMPLEMENTATION
impl crate::converter::Class for Variable {
    fn name(&self) -> &'static str {"Variable"}
    fn locale(&self, code: u8) -> () {match code {
        0 => crate::stdout::debug(&crate::format!(
            "Variable name is \"{}\"",
            &*self.characters
        )),
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn Value> {
        self.locale(0);
        return crate::Box::new(crate::_Variable {
            name: self.characters.clone().into_string()
        });
    }
} impl Variable {
    pub fn new(characters: &str) -> Self {return Variable {
        characters: characters.into()
    }}
}