//
//  START
//

// START -> STRUCT
pub struct Start {
    statements: crate::Box<[u32]>
}

// START -> IMPL
impl crate::Object for Start {}
impl Start {
    pub fn new(statements: &[u32]) -> Self {
        crate::stdout::trace("Creating Start");
        return Start {
            statements: statements.into()
        };
    }
}