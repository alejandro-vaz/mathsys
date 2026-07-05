//^
//^ HEAD
//^

//> HEAD -> STD
use std::collections::{
    HashSet as Set,
    HashMap as Map
};


//^
//^ CONTEXT
//^

//> CONTEXT -> STRUCT
#[derive(Clone, Default)]
pub struct Context<'valid> {
    pub functions: Set<&'valid str>,
    pub dependencies: Map<&'valid str, Set<&'valid str>>
}