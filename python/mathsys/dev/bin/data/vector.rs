//
//  VECTOR
//

// VECTOR -> STRUCT
pub struct Vector {
    values: crate::Box<[u32]>
}

// VECTOR -> IMPLEMENTATION
impl crate::Object for Vector {}
impl Vector {
    pub fn new(values: &[u32]) -> Self {
        crate::stdout::trace("Creating Vector");
        return Vector {
            values: values.into()
        }
    }
}