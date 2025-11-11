//^
//^ NUMBER
//^

//> NUMBER -> STRUCT
pub struct Number {
    value: u32,
    shift: u8
}

//> NUMBER -> IMPLEMENTATION
impl crate::converter::Class for Number {
    fn name(&self) -> &'static str {"Number"}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        return crate::Box::new(crate::_Number {
            value: self.value,
            shift: self.shift
        });
    }
} impl Number {
    pub fn new(value: u32, shift: u8) -> Self {return Number {
        value: value,
        shift: shift
    }}
}