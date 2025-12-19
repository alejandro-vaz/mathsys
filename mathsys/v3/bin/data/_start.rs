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
pub struct _Start {
    pub statements: crate::Box<[u32]>
}

//> START -> IMPLEMENTATION
impl Class for _Start {
    fn name(&self) -> &'static str {"_Start"}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > statements = [{}]",
        self.name(),
        self.statements.iter().map(|id| crate::format!("{}", id)).collect::<crate::Vec<_>>().join(", ")
    ))}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32) -> crate::Box<dyn Value> {
        self.space("Processing", id);
        for &statement in &self.statements {context.process(statement);}
        return crate::Box::new(crate::Nexists {});
    }
}