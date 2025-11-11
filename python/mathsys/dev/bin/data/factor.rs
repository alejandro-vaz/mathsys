//^
//^ FACTOR
//^

//> FACTOR -> STRUCT
pub struct Factor {
    pointer: u32,
    expression: u32
}

//> FACTOR -> IMPLEMENTATION
impl crate::converter::Class for Factor {
    fn name(&self) -> &'static str {"Factor"}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        return crate::Box::new(crate::_Undefined {});
    }
} impl Factor {
    pub fn new(pointer: u32, expression: u32) -> Self {return Factor {
        pointer: pointer,
        expression: expression
    }}
}