//
//  LIMIT
//

// LIMIT -> STRUCT
pub struct Limit {
    variable: u32,
    approach: u32,
    direction: u8,
    pointer: u32,
    exponent: u32
}

// LIMIT -> IMPLEMENTATION
impl crate::Object for Limit {}
impl Limit {
    pub fn new(variable: u32, approach: u32, direction: u8, pointer: u32, exponent: u32) -> Self {
        crate::stdout::trace("Creating Limit");
        return Limit {
            variable: variable,
            approach: approach,
            direction: direction,
            pointer: pointer,
            exponent: exponent
        }
    }
}