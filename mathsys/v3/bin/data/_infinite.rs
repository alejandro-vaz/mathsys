//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ INFINITE
//^

//> INFINITE -> STRUCT
pub struct _Infinite {}

//> INFINITE -> IMPLEMENTATION
impl Class for _Infinite {
    fn name(&self) -> &'static str {"_Infinite"}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > ",
        self.name()
    ))}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32) -> crate::Box<dyn Value> {
        self.space("Processing", id);
        return crate::Box::new(crate::Infinite {
            negative: false
        });
    }
}