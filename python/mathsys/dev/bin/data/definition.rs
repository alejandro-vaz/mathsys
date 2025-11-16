//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


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
    fn locale(&self, code: u8) -> () {match code {
        0 => crate::stdout::debug(&crate::format!(
            "Variable ID is {}",
            self.variable
        )),
        1 => crate::stdout::debug(&crate::format!(
            "Main expression ID is {}",
            self.pointer
        )),
        2 => crate::stdout::space(&crate::format!(
            "[{}] Assigning immutable variable",
            self.name()
        )),
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn Value> {
        self.locale(0);
        self.locale(1);
        context.process(self.variable);
        context.process(self.pointer);
        self.locale(2);
        let pointer = context.read(self.pointer);
        let reference = context.read(self.variable);
        let variable = match reference.id() {
            "_Variable" => crate::runtime::downcast::<crate::_Variable>(&*reference),
            other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
        };
        variable.set(pointer, false, context);
        return crate::Box::new(crate::_Nexists {});
    }
} impl Definition {
    pub fn new(variable: u32, pointer: u32) -> Self {return Definition {
        variable: variable,
        pointer: pointer
    }}
}