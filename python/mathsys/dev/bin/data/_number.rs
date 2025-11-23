//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ NUMBER
//^

//> NUMBER -> STRUCT
pub struct _Number {
    pub value: u32,
    pub shift: u8
}

//> NUMBER -> IMPLEMENTATION
impl Class for _Number {
    fn name(&self) -> &'static str {"_Number"}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > value = {}, shift = {}",
        self.name(),
        self.value,
        self.shift
    ))}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32) -> crate::Box<dyn Value> {
        self.space("Processing", id);
        return crate::Box::new(crate::Number {
            value: self.value,
            shift: self.shift,
            negative: false
        });
    }
}