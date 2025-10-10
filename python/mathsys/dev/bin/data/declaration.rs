//
//  DECLARATION
//

// DECLARATION -> STRUCT
pub struct Declaration {
    variable: u32,
    pointer: u32
}

// DECLARATION -> IMPL
impl crate::converter::Class for Declaration {
    fn name(&self) -> &'static str {"Declaration"}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        return crate::Box::new(crate::_Undefined {});
    }
} impl Declaration {
    pub fn new(variable: u32, pointer: u32) -> Self {return Declaration {
        variable: variable,
        pointer: pointer
    }}
}