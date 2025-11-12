//^
//^ NODE
//^

//> NODE -> STRUCT
pub struct Node {
    pointer: u32
}

//> NODE -> IMPLEMENTATION
impl crate::converter::Class for Node {
    fn name(&self) -> &'static str {"Node"}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        return crate::Box::new(crate::_Undefined {});
    }
} impl Node {
    pub fn new(pointer: u32) -> Self {return Node {
        pointer: pointer
    }}
}