//^
//^ START
//^

//> START -> STRUCT
pub struct Start {
    statements: crate::Box<[u32]>
}

//> START -> IMPLEMENTATION
impl crate::converter::Class for Start {
    fn name(&self) -> &'static str {"Start"}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        self.locale(0);
        self.locale(1);
        for &statement in &self.statements {context.process(statement);}
        self.locale(2);
        self.locale(3);
        return crate::Box::new(crate::_Undefined {});
    }
} impl Start {
    pub fn new(statements: &[u32]) -> Self {return Start {
        statements: statements.into()
    }}
    fn locale(&self, code: u8) -> () {
        match code {
            0 => {crate::ALLOCATOR.tempSpace(|| {crate::stdout::debug(&crate::format!(
                "There {} {} statement{}",
                if self.statements.len() == 1 {"is"} else {"are"},
                self.statements.len(),
                if self.statements.len() == 1 {""} else {"s"}
            ))})},
            1 => {crate::stdout::trace("Iterating to evaluate all statements")}
            2 => {crate::stdout::space("Shutdown")},
            3 => {crate::ALLOCATOR.tempSpace(|| {crate::stdout::debug(&crate::format!(
                "{} statement{} evaluated correctly",
                self.statements.len(),
                if self.statements.len() == 1 {""} else {"s"}
            ))})},
            _ => {crate::stdout::crash(4)},
        }
    }
}