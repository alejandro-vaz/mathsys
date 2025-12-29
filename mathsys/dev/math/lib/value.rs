//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    fmt,
    debug
};


//^
//^ VALUE
//^

//> VALUE -> TRAIT
pub trait Value: fmt::Display + fmt::Debug {fn data(&self) -> () {debug(format!(
    "{:?}",
    self
))}}