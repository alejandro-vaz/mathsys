//
//  DECLARATION
//

// DECLARATION -> STRUCT
pub struct Declaration {
    variable: u32,
    pointer: u32
}

// DECLARATION -> IMPL
impl crate::Object for Declaration {}
impl Declaration {
    pub fn new(variable: u32, pointer: u32) -> Self {
        crate::stdout::trace("Creating Declaration");
        return Declaration {
            variable: variable,
            pointer: pointer
        }
    }
}