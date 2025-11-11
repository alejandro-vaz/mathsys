//^
//^ VECTOR
//^

//> VECTOR -> STRUCT
pub struct Vector {
    values: crate::Box<[u32]>
}

//> VECTOR -> IMPLEMENTATION
impl crate::converter::Class for Vector {
    fn name(&self) -> &'static str {"Vector"}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        return crate::Box::new(crate::_Undefined {})
    }
} impl Vector {
    pub fn new(values: &[u32]) -> Self {return Vector {
        values: values.into()
    }}
}