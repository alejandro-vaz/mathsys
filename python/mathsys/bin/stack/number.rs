//
//  IMPORTS
//

// IMPORTS -> BLOCK
unsafe extern "C" {
    fn numberAdd(first: usize, second: usize) -> usize;
}


//
//  WRAPPERS
//

// WRAPPERS -> ADD
pub unsafe fn add(first: usize, second: usize) -> usize {numberAdd(first, second)}