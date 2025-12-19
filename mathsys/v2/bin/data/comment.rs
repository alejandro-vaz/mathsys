//
//  COMMENT
//

// COMMENT -> STRUCT
pub struct Comment {
    characters: crate::Box<str>,
}

// COMMENT -> IMPLEMENTATION
impl crate::Object for Comment {}
impl Comment {
    pub fn new(characters: &str) -> Self {
        crate::stdout::trace("Creating Comment");
        return Comment {
            characters: characters.into()
        }
    }
}