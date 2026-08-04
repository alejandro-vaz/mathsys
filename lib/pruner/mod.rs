//^
//^ HEAD
//^

//> HEAD -> STD
use std::collections::{
    HashSet as Set,
    VecDeque
};

//> HEAD -> PETGRAPH
use petgraph::{
    Incoming,
    dot::{
        Dot,
        Config
    },
    Graph,
    Directed
};

//> HEAD -> CRATE
use crate::parser::forest::Forest;


//^
//^ PRUNER
//^

//> PRUNER -> FUNCTION
pub fn prune<'valid>(forest: Forest<'valid>) -> () {
    //let mut seen = Set::new();
    //let mut frontier = VecDeque::new();
    //frontier.extend(parse.accepted.iter().cloned());
    //while let Some(trace) = frontier.pop_front() {
    //    if !seen.insert(trace) {continue}
    //    for node in parse.path.neighbors_directed(trace.0, Incoming) {
    //        frontier.push_back(Trace(node));
    //    }
    //}
    //parse.path.retain_nodes(|_, node| seen.contains(&Trace(node)));
    dbg!(&forest.accepted);
    println!(
        "{:?}",
        Dot::with_config(&forest.graph, &[])
    );
    //return Forest {
    //    start: parse.start.0,
    //    path: parse.path
    //};
}