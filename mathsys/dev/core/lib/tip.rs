//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::stdout::{debug, space};


//^
//^ TIP
//^

//> TIP -> TRAIT
pub trait Tip: crate::Display + crate::Debug {
    fn data(&self) -> () {debug(format!(
        "{} > {:?}",
        self, self
    ))}
    fn space<Type: crate::Display>(&self, message: Type, id: u32) -> () {space(format!(
        "{{{}{}}} {}",
        id, self, message
    )); self.data()}
}