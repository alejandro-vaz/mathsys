//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::stdout::{class, space};
use crate::types::Pointer;


//^
//^ TIP
//^

//> TIP -> TRAIT
pub trait Tip: crate::Display + crate::Debug {
    fn data(&self) -> () {class(format!(
        "{} > {:?}",
        self, self
    ))}
    fn section<Type: crate::Display>(&self, message: Type, id: Pointer) -> () {space(format!(
        "{{{}{}}} {}",
        id.0, self, message
    )); self.data()}
}