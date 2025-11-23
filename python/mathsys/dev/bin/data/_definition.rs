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
pub struct _Definition {
    pub object: u8,
    pub variable: u32,
    pub pointer: u32
}

//> DEFINITION -> IMPLEMENTATION
impl Class for _Definition {
    fn name(&self) -> &'static str {"_Definition"}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > object = {}, variable = {}, pointer = {}",
        self.name(),
        self.object,
        self.variable,
        self.pointer
    ))}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32) -> crate::Box<dyn Value> {
        self.space("Processing", id);
        context.process(self.variable);
        context.process(self.pointer);
        self.space("Setting immutable variable", id);
        let pointer = context.read(self.pointer);
        let mut reference = context.read(self.variable);
        let variable = crate::runtime::mutcast::<crate::Variable>(&mut *reference);
        variable.set(pointer, false, context, self.object);
        return crate::Box::new(crate::Nexists {});
    }
}