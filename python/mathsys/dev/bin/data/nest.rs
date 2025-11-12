//^
//^ NEST
//^

//> NEST -> STRUCT
pub struct Nest {
    pointer: u32
}

//> NEST -> IMPLEMENTATION
impl crate::converter::Class for Nest {
    fn name(&self) -> &'static str {"Nest"}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        return crate::Box::new(crate::_Undefined {});
    }
} impl Nest {
    pub fn new(pointer: u32) -> Self {return Nest {
        pointer: pointer
    }}
}