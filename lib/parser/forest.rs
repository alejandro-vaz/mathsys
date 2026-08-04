//^
//^ HEAD
//^

//> HEAD -> PETGRAPH
use petgraph::{
    Graph,
    graph::{
        NodeIndex,
        Edges
    },
    Directed
};

//> HEAD -> SUPER
use super::{
    parsed::Parsed,
    rule::Rule
};

//> HEAD -> CRATE
use crate::tokenizer::token::Token;

//> HEAD -> STD
use std::collections::{
    HashMap as Map,
    HashSet as Set
};


//^
//^ FOREST
//^

//> FOREST -> STRUCT
#[derive(Default)]
pub struct Forest<'valid> {
    graph: Graph<Parsed<'valid>, ()>,
    nodes: Map<Parsed<'valid>, NodeIndex>,
    accepted: Set<NodeIndex>
}

//> FOREST -> IMPLEMENTATION
impl<'valid> Forest<'valid> {
    pub fn shift(&mut self, token: &'valid Token<'valid>, index: usize) -> NodeIndex {
        return *self.nodes.entry(Parsed::Terminal {
            token: token,
            index: index
        }).or_insert_with(|| self.graph.add_node(Parsed::Terminal {
            token: token,
            index: index
        }));
    }
    pub fn accept(&mut self, nodes: Edges<NodeIndex, Directed>) -> () {
        self.accepted.extend(nodes.map(|reference| *reference.weight()));
    }
}