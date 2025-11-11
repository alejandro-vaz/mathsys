//^
//^ INFINITE
//^

//> INFINITE -> STRUCT
pub struct Infinite {}

//> INFINITE -> IMPLEMENTATION
impl crate::converter::Class for Infinite {
    fn name(&self) -> &'static str {"Infinite"}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        return crate::Box::new(crate::_Infinity {});
    }
} impl Infinite {
    pub fn new() -> Self {return Infinite {}}
}