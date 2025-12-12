//^
//^ TIP
//^

//> TIP -> TRAIT
pub trait Tip: crate::Display + crate::Debug {
    fn data(&self) -> () {crate::stdout::debug(format!(
        "{} > {:?}",
        self, self
    ))}
    fn space<Type: crate::Display>(&self, message: Type, id: u32) -> () {crate::stdout::space(format!(
        "{{{}{}}} {}",
        id, self, message
    )); self.data()}
}