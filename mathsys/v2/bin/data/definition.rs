//
//  DEFINITION
//

// DEFINITION -> STRUCT
pub struct Definition {
    variable: u32,
    pointer: u32
}

// DEFINITION -> IMPL
impl crate::Object for Definition {}
impl Definition {
    pub fn new(variable: u32, pointer: u32) -> Self {
        crate::stdout::trace("Creating Definition");
        return Definition {
            variable: variable,
            pointer: pointer
        }
    }
}