//^
//^ HEAD
//^

//> HEAD -> PETGRAPH
use petgraph::{
    Graph, 
    Incoming, 
    graph::NodeIndex, 
    visit::EdgeRef
};

//> HEAD -> STD
use std::collections::{
    HashMap as Map, 
    HashSet as Set, 
    VecDeque,
    hash_map::Entry
};

//> HEAD -> SUPER
use super::{
    rule::Rule,
    forest::Forest,
    tables::{
        GOTO,
        ACTION
    },
    action::Action
};

//> HEAD -> CRATE
use crate::tokenizer::token::Token;

//> HEAD -> LIBUTILS
use libutils::stack_array::Array;

//> HEAD -> CORE
use core::mem::take;


//^
//^ MACHINE
//^

//> MACHINE -> STRUCT
pub struct Machine<'valid> {
    graph: Graph<usize, NodeIndex>,
    states: Map<(usize, usize), NodeIndex>,
    processed: Set<NodeIndex>,
    heads: VecDeque<NodeIndex>,
    following: Set<NodeIndex>,
    forest: Forest<'valid>,
    index: usize
}

//> MACHINE -> DEFAULT
impl<'valid> Default for Machine<'valid> {
    fn default() -> Self {
        let mut graph = Graph::default();
        let node = graph.add_node(0);
        return Self {
            graph: graph,
            states: Map::from([((0, 0), node)]),
            processed: Set::from([node]),
            heads: VecDeque::from([node]),
            following: Set::new(),
            forest: Forest::default(),
            index: 0
        }
    }
}

//> MACHINE -> IMPLEMENTATION
impl<'valid> Machine<'valid> {
    fn shift(
        &mut self, 
        state: NodeIndex, 
        token: &'valid Token<'valid>,
        goto: &'static usize
    ) -> () {
        let to = *self.states.entry((*goto, self.index + 1)).or_insert_with(|| {
            self.graph.add_node(*goto)
        });
        let node = self.forest.shift(token, self.index);
        self.graph.update_edge(state, to, node);
        self.following.insert(to);
    }
    fn reduce(
        &mut self, 
        state: NodeIndex, 
        length: &'static usize,
        rule: &'static Rule
    ) -> () {
        let mut frontier = Vec::from([(state, Array::new())]);
        for _ in 0..*length {
            for (node, children) in take(&mut frontier) {
                for edge in self.graph.edges_directed(node, Incoming) {
                    let mut next = children.clone();
                    next.push(*edge.weight());
                    frontier.push((edge.source(), next));
                }
            }
        }
        for (from, children) in frontier {
            let Some(node) = self.forest.reduce(rule, children, self.index) else {continue};
            let goto = GOTO[self.graph[from]][rule];
            let to = *self.states.entry((goto, self.index)).or_insert_with(|| {
                self.graph.add_node(goto)
            });
            self.graph.update_edge(from, to, node);
            if self.processed.insert(to) {self.heads.push_back(to)}
        }
    }
    pub fn finish(self) -> Forest<'valid> {return self.forest}
    pub fn advance(&mut self) -> () {
        self.heads.extend(self.following.drain());
        self.processed.clear();
        self.index += 1;
    }
    pub fn pass(&mut self, token: &'valid Token<'valid>) -> () {
        let name = token.as_ref();
        while let Some(state) = self.heads.pop_front() {for action in ACTION[
            self.graph[state]
        ].get(name).map(Array::as_ref).unwrap_or_default() {match action {
            Action::Reduce {rule, length} => self.reduce(state, length, rule),
            Action::Shift {goto} => self.shift(state, token, goto),
        }}}
    }
}