//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ FACTOR
//^

//> FACTOR -> STRUCT
pub struct _Factor {
    pub pointer: u32,
    pub expression: u32
}

//> FACTOR -> IMPLEMENTATION
impl Class for _Factor {
    fn name(&self) -> &'static str {"_Factor"}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > pointer = {}, expression = {}",
        self.name(),
        self.pointer,
        self.expression
    ))}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32) -> crate::Box<dyn Value> {
        return crate::Box::new(crate::Undefined {});
    }
}