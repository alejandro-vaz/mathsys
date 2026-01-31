//
//  DEBUG
//

// DEBUG -> STRUCT
pub struct Debug {}

// DEBUG -> IMPL
impl crate::Object for Debug {}
impl Debug {
    pub fn new() -> Self {
        crate::stdout::trace("Creating Debug");
        return Debug {}
    }
}