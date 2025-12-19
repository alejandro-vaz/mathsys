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
pub struct _Node {
    pub pointer: u32
}

//> NODE -> IMPLEMENTATION
impl Class for _Node {
    fn name(&self) -> &'static str {"_Node"}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > pointer = {}",
        self.name(),
        self.pointer
    ))}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32) -> crate::Box<dyn Value> {
        self.space("Processing", id);
        context.process(self.pointer);
        return crate::Box::new(crate::Undefined {});
    }
}