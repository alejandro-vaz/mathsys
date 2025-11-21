//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ DECLARATION
//^

//> DECLARATION -> STRUCT
pub struct _Declaration {
    object: u8,
    variable: u32,
    pointer: u32
}

//> DECLARATION -> IMPLEMENTATION
impl Class for _Declaration {
    fn name(&self) -> &'static str {"_Declaration"}
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
            "[{}] Assigning mutable variable",
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
        let mut reference = context.read(self.variable);
        let variable = crate::runtime::mutcast::<crate::Variable>(&mut *reference);
        variable.set(pointer, true, context);
        return crate::Box::new(crate::Nexists {});
    }
} impl _Declaration {
    pub fn new(object: u8, variable: u32, pointer: u32) -> Self {return _Declaration {
        object: object,
        variable: variable,
        pointer: pointer
    }}
}