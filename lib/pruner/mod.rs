//^
//^ HEAD
//^

//> HEAD -> PETGRAPH
use petgraph::{
    Outgoing,
    dot::{
        Dot,
        Config
    }
};

//> HEAD -> PARSER
use crate::parser::forest::Forest;


//^
//^ PRUNER
//^

//> PRUNER -> FUNCTION
pub fn prune<'valid>(forest: Forest<'valid>) -> () {
    dbg!(forest.roots());
}
