//
//  FACTOR
//

// FACTOR -> STRUCT
pub struct Factor {
    pointer: u32,
    expression: u32
}

// FACTOR -> IMPLEMENTATION
impl crate::Object for Factor {}
impl Factor {
    pub fn new(pointer: u32, expression: u32) -> Self {
        crate::stdout::trace("Creating Factor");
        return Factor {
            pointer: pointer,
            expression: expression
        }
    }
}