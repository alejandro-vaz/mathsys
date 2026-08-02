//^
//^ HEAD
//^

//> HEAD -> PETGRAPH
use petgraph::{
    Graph,
    graph::NodeIndex
};

//> HEAD -> CRATE
use crate::parser::action::Action;


//^
//^ HEAD
//^

//> HEAD -> FOREST
pub struct Forest {
    pub start: NodeIndex,
    pub path: Graph<&'static Action, ()>
}