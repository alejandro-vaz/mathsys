//
//  VARIABLE
//

// VARIABLE -> STRUCT
pub struct Variable {
    characters: crate::Box<str>
}

// VARIABLE -> IMPLEMENTATION
impl crate::Object for Variable {}
impl Variable {
    pub fn new(characters: &str) -> Self {
        crate::stdout::trace("Creating Variable");
        return Variable {
            characters: characters.into()
        }
    }
}