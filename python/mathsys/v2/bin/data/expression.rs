//
//  EXPRESSION
//

// EXPRESSION -> STRUCT
pub struct Expression {
    terms: crate::Box<[u32]>,
    signs: crate::Box<[u8]>
}

// EXPRESSION -> IMPLEMENTATION
impl crate::Object for Expression {}
impl Expression {
    pub fn new(terms: &[u32], signs: &[u8]) -> Self {
        crate::stdout::trace("Creating Expression");
        return Expression {
            terms: terms.into(),
            signs: signs.into()
        }
    }
}