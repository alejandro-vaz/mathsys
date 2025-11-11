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
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        return crate::Box::new(crate::_Undefined {});
    }
} impl Comment {
    pub fn new(characters: &str) -> Self {return Comment {
        characters: characters.into()
    }}
}