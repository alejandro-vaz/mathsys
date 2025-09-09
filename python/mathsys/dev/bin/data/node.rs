//
//  NODE
//

// NODE -> STRUCT
pub struct Node {
    pointer: u32
}

// NODE -> IMPLEMENTATION
impl crate::Object for Node {}
impl Node {
    pub fn new(pointer: u32) -> Self {
        crate::stdout::trace("Creating Node");
        return Node {
            pointer: pointer
        }
    }
}