//
//  NUMBER
//

// NUMBER -> STRUCT
pub struct Number {
    value: u32,
    shift: u8
}

// NUMBER -> IMPLEMENTATION
impl crate::Object for Number {}
impl Number {
    pub fn new(value: u32, shift: u8) -> Self {
        crate::stdout::trace("Creating Number");
        return Number {
            value: value,
            shift: shift
        }
    }
}