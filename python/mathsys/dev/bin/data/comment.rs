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
pub struct Comment {
    characters: crate::Box<str>,
}

//> COMMENT -> IMPLEMENTATION
impl crate::converter::Class for Comment {
    fn name(&self) -> &'static str {"Comment"}
    fn locale(&self, code: u8) -> () {match code {
        0 => crate::stdout::debug(&crate::format!(
            "{}",
            self.characters.clone()
        )),
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn Value> {
        self.locale(0);
        return crate::Box::new(crate::_Nexists {});
    }
} impl Comment {
    pub fn new(characters: &str) -> Self {return Comment {
        characters: characters.into()
    }}
}