//
//  INFINITE
//

// INFINITE -> STRUCT
pub struct Infinite {}

// INFINITE -> IMPLEMENTATION
impl crate::Object for Infinite {}
impl Infinite {
    pub fn new() -> Self {
        crate::stdout::trace("Creating Infinite");
        return Infinite {}
    }
}