//^
//^ DEBUG
//^

//> DEBUG -> STRUCT
pub struct Debug {}

//> DEBUG -> IMPLEMENTATION
impl crate::converter::Class for Debug {
    fn name(&self) -> &'static str {"Debug"}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        return crate::Box::new(crate::_Undefined {});
    }
} impl Debug {
    pub fn new() -> Self {return Debug {}}
}