//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Map, Set
};


//^
//^ RUNTIME
//^

//> RUNTIME -> OBJECT
pub struct Runtime {
    pub immutables: Set<String>,
    pub variables: Map<String, ()>
} impl Runtime {pub fn new() -> Self {return Runtime {
    immutables: Set::new(),
    variables: Map::new()
}}}