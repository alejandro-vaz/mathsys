//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ EQUATION
//^

//> EQUATION -> STRUCT
pub struct _Equation {
    pub left: u32,
    pub right: u32
}

//> EQUATION -> IMPLEMENTATION
impl Class for _Equation {
    fn name(&self) -> &'static str {"_Equation"}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > left = {}, right = {}",
        self.name(),
        self.left,
        self.right
    ))}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32) -> crate::Box<dyn Value> {
        self.space("Processing", id);
        context.process(self.left);
        context.process(self.right);
        self.space("Checking if both sides are equal", id);
        let left = context.read(self.left);
        let right = context.read(self.right);
        left.equiv(right);
        return crate::Box::new(crate::Nexists {});
    }
}