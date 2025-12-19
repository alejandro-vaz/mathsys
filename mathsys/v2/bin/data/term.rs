//
//  TERM
//

// TERM -> STRUCT
pub struct Term {
    numerator: crate::Box<[u32]>,
    denominator: crate::Box<[u32]>
}

// TERM -> IMPLEMENTATION
impl crate::Object for Term {}
impl Term {
    pub fn new(numerator: &[u32], denominator: &[u32]) -> Self {
        crate::stdout::trace("Creating Term");
        return Term {
            numerator: numerator.into(),
            denominator: denominator.into()
        }
    }
}