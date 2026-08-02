//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    trace::Trace,
    action::Action
};

//> HEAD -> STD
use std::collections::HashSet as Set;

//> HEAD -> PETGRAPH
use petgraph::Graph;


//^
//^ PARSE
//^

//> PARSE -> STRUCT
pub struct Parse {
    pub start: Trace,
    pub accepted: Set<Trace>,
    pub path: Graph<&'static Action, ()>
}