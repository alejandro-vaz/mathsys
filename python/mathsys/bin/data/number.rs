//
//  HEAD
//

// HEAD -> ALLOCATOR
use crate::GlobalAlloc;


//
//  STRUCT
//

// STRUCT -> FACTOR
pub struct Number {
    pub signs: *mut u8,
    pub precision: *mut u8,
    pub value: *mut u8
}


//
//  IMPLEMENTATION
//

// IMPLEMENTATION -> FACTOR
impl Number {
    pub unsafe fn new() -> Self {
        let object = Number {
            signs: crate::ALLOCATOR.alloc(crate::Layout::new::<u8>()),
            precision: crate::ALLOCATOR.alloc(crate::Layout::new::<u8>()),
            value: crate::ALLOCATOR.alloc(crate::Layout::new::<u8>()),
        };
        crate::stdout::trace("New Number created.");
        return object;
    }
}