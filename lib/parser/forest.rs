//^
//^ HEAD
//^

//> HEAD -> PETGRAPH
use petgraph::{
    Graph,
    graph::NodeIndex
};

//> HEAD -> SUPER
use super::{
    parsed::Parsed,
    rule::Rule,
    constants::DERIVATION_LENGTH
};

//> HEAD -> CRATE
use crate::tokenizer::token::Token;

//> HEAD -> STD
use std::collections::{
    HashMap as Map,
    HashSet as Set
};

//> HEAD -> LIBUTILS
use libutils::stack_array::Array;


//^
//^ FOREST
//^

//> FOREST -> STRUCT
#[derive(Default)]
pub struct Forest<'valid> {
    pub graph: Graph<Parsed<'valid>, ()>,
    pub nodes: Map<Parsed<'valid>, NodeIndex>,
    pub accepted: Set<NodeIndex>
}

//> FOREST -> IMPLEMENTATION
impl<'valid> Forest<'valid> {
    pub fn shift(&mut self, token: &'valid Token<'valid>, index: usize) -> NodeIndex {
        let parsed = Parsed::Terminal {
            token: token, 
            index: index 
        };
        return *self.nodes.entry(parsed).or_insert_with(|| self.graph.add_node(parsed));
    }
    pub fn reduce(
        &mut self, 
        rule: &'static Rule, 
        children: Array<NodeIndex, DERIVATION_LENGTH>,
        index: usize
    ) -> Option<NodeIndex> {
        let start = children.last().map(|&node| match self.graph[node] {
            Parsed::NonTerminal {span, ..} => span.start,
            Parsed::Terminal {index, ..} => index
        }).unwrap_or(index);
        let parsed = Parsed::NonTerminal {
            rule: rule, 
            span: start..index
        };
        let node = *self.nodes.entry(parsed).or_insert_with(|| self.graph.add_node(parsed));
        for child in children {self.graph.update_edge(node, child, ());}
        return if let Rule::usize(0) = rule {
            self.accepted.insert(node);
            None
        } else {Some(node)}
    }
}