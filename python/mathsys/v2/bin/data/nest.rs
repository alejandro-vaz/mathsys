//
//  NEST
//

// NEST -> STRUCT
pub struct Nest {
    pointer: u32
}

// NEST -> IMPLEMENTATION
impl crate::Object for Nest {}
impl Nest {
    pub fn new(pointer: u32) -> Self {
        crate::stdout::trace("Creating Nest");
        return Nest {
            pointer: pointer
        }
    }
}