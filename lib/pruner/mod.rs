//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod forest;

//> HEAD -> CRATE
use crate::parser::{
    parse::Parse,
    trace::Trace
};

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
    }
};

//> HEAD -> FOREST
use forest::Forest;


//^
//^ PRUNER
//^

//> PRUNER -> FUNCTION
pub fn prune(mut parse: Parse) -> Forest {
    println!("{:?}", parse.accepted);
    let mut seen = Set::new();
    let mut frontier = VecDeque::new();
    frontier.extend(parse.accepted.iter().cloned());
    while let Some(trace) = frontier.pop_front() {
        if !seen.insert(trace) {continue}
        for node in parse.path.neighbors_directed(trace.0, Incoming) {
            frontier.push_back(Trace(node));
        }
    }
    parse.path.retain_nodes(|_, node| seen.contains(&Trace(node)));
    println!(
        "{:?}",
        Dot::with_config(&parse.path, &[Config::EdgeNoLabel])
    );
    return Forest {
        start: parse.start.0,
        path: parse.path
    };
}