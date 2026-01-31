//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::stdout::{class, space};


//^
//^ TIP
//^

//> TIP -> TRAIT
pub trait Tip: crate::Display + crate::Debug {
    fn data(&self) -> () {class(format!(
        "{} > {:?}",
        self, self
    ))}
    fn space<Type: crate::Display>(&self, message: Type, id: u32) -> () {space(format!(
        "{{{}{}}} {}",
        id, self, message
    )); self.data()}
}