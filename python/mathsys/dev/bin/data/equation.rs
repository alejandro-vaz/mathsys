//^
//^ EQUATION
//^

//> EQUATION -> STRUCT
pub struct Equation {
    left: u32,
    right: u32
}

//> EQUATION -> IMPLEMENTATION
impl crate::converter::Class for Equation {
    fn name(&self) -> &'static str {"Equation"}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        return crate::Box::new(crate::_Undefined {});
    }
    fn locale(&self, code: u8) -> () {match code {
        _ => {crate::stdout::crash(crate::stdout::Code::LocaleNotFound)}
    }}
} impl Equation {pub fn new(left: u32, right: u32) -> Self {return Equation {
    left: left,
    right: right
}}}