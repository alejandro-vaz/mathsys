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

//> HEAD -> CRATE
use crate::parser::forest::Forest;

//> HEAD -> STD
use std::collections::HashSet as Set;


//^
//^ PRUNER
//^

//> PRUNER -> FUNCTION
pub fn prune<'valid>(mut forest: Forest<'valid>) -> () {
    dbg!(&forest.accepted);
    //println!(
    //    "{:?}",
    //    Dot::with_config(&forest.graph, &[Config::EdgeNoLabel])
    //);
    let mut seen = Set::new();
    let mut frontier = Vec::from_iter(forest.accepted);
    while let Some(node) = frontier.pop() {
        if !seen.insert(node) {continue}
        frontier.extend(forest.graph.neighbors_directed(node, Outgoing));
    }
    forest.graph.retain_nodes(|_, node| seen.contains(&node));
    println!(
        "{:?}",
        Dot::with_config(&forest.graph, &[Config::EdgeNoLabel])
    );
}