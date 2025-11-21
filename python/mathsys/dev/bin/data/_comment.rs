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
    characters: crate::Box<str>,
}

//> COMMENT -> IMPLEMENTATION
impl Class for _Comment {
    fn name(&self) -> &'static str {"_Comment"}
    fn locale(&self, code: u8) -> () {match code {
        0 => crate::stdout::debug(&crate::format!(
            "{}",
            self.characters.clone()
        )),
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn Value> {
        self.locale(0);
        return crate::Box::new(crate::Nexists {});
    }
} impl _Comment {
    pub fn new(characters: &str) -> Self {return _Comment {
        characters: characters.into()
    }}
}