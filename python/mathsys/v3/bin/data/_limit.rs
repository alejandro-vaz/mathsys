//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ LIMIT
//^

//> LIMIT -> STRUCT
pub struct _Limit {
    pub variable: u32,
    pub approach: u32,
    pub direction: u8,
    pub pointer: u32,
    pub exponent: u32
}

//> LIMIT -> IMPLEMENTATION
impl Class for _Limit {
    fn name(&self) -> &'static str {"_Limit"}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > variable = {}, approach = {}, direction = {}, pointer = {}, exponent = {}",
        self.name(),
        self.variable,
        self.approach,
        self.direction,
        self.pointer,
        self.exponent
    ))}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32) -> crate::Box<dyn Value> {
        return crate::Box::new(crate::Undefined {});
    }
}