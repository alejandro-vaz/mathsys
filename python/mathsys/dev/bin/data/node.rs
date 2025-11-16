//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


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
    fn locale(&self, code: u8) -> () {match code {
        0 => crate::stdout::debug(&crate::format!(
            "Expression to evaluate has an ID of {}",
            self.pointer
        )),
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn Value> {
        self.locale(0);
        context.process(self.pointer);
        return crate::Box::new(crate::_Undefined {});
    }
} impl Node {
    pub fn new(pointer: u32) -> Self {return Node {
        pointer: pointer
    }}
}