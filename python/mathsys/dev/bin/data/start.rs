//
//  START
//

// START -> STRUCT
pub struct Start {
    statements: crate::Box<[u32]>
}

// START -> IMPL
impl crate::converter::Class for Start {
    fn name(&self) -> &'static str {"Start"}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        crate::stdout::space("Evaluating start");
        crate::ALLOCATOR.tempSpace(|| {crate::stdout::debug(&crate::format!(
            "There {} {} statement{}",
            if self.statements.len() == 1 {"is"} else {"are"},
            self.statements.len(),
            if self.statements.len() == 1 {""} else {"s"}
        ))});
        for &statement in &self.statements {
            crate::ALLOCATOR.tempSpace(|| {crate::stdout::trace(&crate::format!(
                "Processing statement with id {}",
                statement
            ))});
            context.process(statement);
        }
        return crate::Box::new(crate::_Undefined {});
    }
} impl Start {
    pub fn new(statements: &[u32]) -> Self {return Start {
        statements: statements.into()
    }}
}