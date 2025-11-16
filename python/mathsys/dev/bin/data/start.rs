//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


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
    fn locale(&self, code: u8) -> () {match code {
        0 => crate::stdout::debug(&crate::format!(
            "There {} {} statement{}",
            if self.statements.len() == 1 {"is"} else {"are"},
            self.statements.len(),
            if self.statements.len() == 1 {""} else {"s"}
        )),
        1 => crate::stdout::debug(&crate::format!(
            "Statement ID{} {} {}",
            if self.statements.len() == 1 {""} else {"s"},
            if self.statements.len() == 1 {"is"} else {"are"},
            self.statements.iter().map(|id| crate::format!("{}", id)).collect::<crate::Vec<_>>().join(", ")
        )),
        2 => crate::stdout::space(&crate::format!(
            "[{}] Shutdown",
            self.name()
        )),
        3 => crate::stdout::debug(&crate::format!(
            "{} statement{} evaluated correctly",
            self.statements.len(),
            if self.statements.len() == 1 {""} else {"s"}
        )),
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn Value> {
        self.locale(0);
        self.locale(1);
        for &statement in &self.statements {context.process(statement);}
        self.locale(2);
        self.locale(3);
        return crate::Box::new(crate::_Nexists {});
    }
} impl Start {
    pub fn new(statements: &[u32]) -> Self {return Start {
        statements: statements.into()
    }}
}