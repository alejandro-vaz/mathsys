//^
//^ DECLARATION
//^

//> DECLARATION -> STRUCT
pub struct Declaration {
    variable: u32,
    pointer: u32
}

//> DECLARATION -> IMPLEMENTATION
impl crate::converter::Class for Declaration {
    fn name(&self) -> &'static str {"Declaration"}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        self.locale(0);
        self.locale(1);
        context.process(self.variable);
        context.process(self.pointer);
        self.locale(2);
        let reference = context.read(self.variable);
        let variable = match reference.id() {
            "Variable" => crate::runtime::downcast::<crate::_Variable>(&*reference),
            _ => crate::stdout::crash(1)
        };
        variable.set(context.read(self.pointer), true, context);
        return crate::Box::new(crate::_Undefined {});
    }
} impl Declaration {
    pub fn new(variable: u32, pointer: u32) -> Self {return Declaration {
        variable: variable,
        pointer: pointer
    }}
    fn locale(&self, code: u8) -> () {
        match code {
            0 => {crate::ALLOCATOR.tempSpace(|| {crate::stdout::debug(&crate::format!(
                "Variable ID is {}",
                self.variable
            ))})},
            1 => {crate::ALLOCATOR.tempSpace(|| {crate::stdout::debug(&crate::format!(
                "Main expression ID is {}",
                self.pointer
            ))})},
            2 => {crate::stdout::space("Assigning mutable variable")},
            _ => {}
        }
    }
}