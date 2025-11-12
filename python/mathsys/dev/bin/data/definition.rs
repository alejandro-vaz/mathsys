//^
//^ DEFINITION
//^

//> DEFINITION -> STRUCT
pub struct Definition {
    variable: u32,
    pointer: u32
}

//> DEFINITION -> IMPLEMENTATION
impl crate::converter::Class for Definition {
    fn name(&self) -> &'static str {"Definition"}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        return crate::Box::new(crate::_Undefined {});
    }
} impl Definition {
    pub fn new(variable: u32, pointer: u32) -> Self {return Definition {
        variable: variable,
        pointer: pointer
    }}
}