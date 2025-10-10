//
//  VARIABLE
//

// VARIABLE -> STRUCT
pub struct Variable {
    characters: crate::Box<str>
}

// VARIABLE -> IMPLEMENTATION
impl crate::converter::Class for Variable {
    fn name(&self) -> &'static str {"Variable"}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        crate::stdout::trace("Getting variable name");
        let name = self.characters.clone();
        crate::ALLOCATOR.tempSpace(|| {crate::stdout::debug(&crate::format!(
            "Variable name is '{}'",
            name
        ))});
        return crate::Box::new(crate::_Variable {
            name: name.into_string()
        });
    }
} impl Variable {
    pub fn new(characters: &str) -> Self {return Variable {
        characters: characters.into()
    }}
}