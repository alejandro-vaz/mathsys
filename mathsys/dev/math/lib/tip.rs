//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    fmt,
    class,
    Pointer,
    space
};


//^
//^ TIP
//^

//> TIP -> TRAIT
pub trait Tip: fmt::Display + fmt::Debug {
    fn data(&self) -> () {class(format!(
        "{} > {:?}",
        self, self
    ))}
    fn section<Type: fmt::Display>(&self, message: Type, id: Pointer) -> () {space(format!(
        "{{{}{}}} {}",
        id.0, self, message
    )); self.data()}
}