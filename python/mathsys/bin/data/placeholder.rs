//
//  HEAD
//

// HEAD -> ALLOCATOR
use crate::GlobalAlloc;


//
//  STRUCT
//

// STRUCT -> PLACEHOLDER
pub struct Placeholder {}

//
//  IMPLEMENTATION
//

// IMPLEMENTATION -> PLACEHOLDER
impl Placeholder {
    pub unsafe fn new() -> Self {
        let object = Placeholder {};
        crate::stdout::trace("New Placeholder created.");
        return object;
    }
}