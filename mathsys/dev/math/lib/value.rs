//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::stdout::debug;


//^
//^ VALUE
//^

//> VALUE -> TRAIT
pub trait Value: crate::Display + crate::Debug {fn data(&self) -> () {debug(format!(
    "{} > {:?}",
    self, self
))}}