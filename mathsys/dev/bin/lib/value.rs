//^
//^ VALUE
//^

//> VALUE -> TRAIT
pub trait Value: crate::Display + crate::Debug {fn data(&self) -> () {crate::stdout::debug(format!(
    "{} > {:?}",
    self, self
))}}