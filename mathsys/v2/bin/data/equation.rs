//
//  EQUATION
//

// EQUATION -> STRUCT
pub struct Equation {
    left: u32,
    right: u32
}

// EQUATION -> IMPLEMENTATION
impl crate::Object for Equation {}
impl Equation {
    pub fn new(left: u32, right: u32) -> Self {
        crate::stdout::trace("Creating Equation");
        return Equation {
            left: left,
            right: right
        }
    }
}