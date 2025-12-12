//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ USE
//^

//> USE -> STRUCT
pub struct _Use {
    pub name: crate::Box<str>,
    pub module: u32
}

//> USE -> IMPLEMENTATION
impl Class for _Use {
    fn name(&self) -> &'static str {"_Use"}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > name = \"{}\", module = {}",
        self.name(),
        &self.name,
        self.module
    ))}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32) -> crate::Box<dyn Value> {
        self.space("Processing", id);
        if self.module != 0 {context.process(self.module)}
        return crate::Box::new(crate::Nexists {});
    }
}