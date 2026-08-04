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
    VecDeque
};

//> HEAD -> SUPER
use super::{
    rule::Rule,
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
pub struct Machine {
    graph: Graph<usize, ()>,
    states: Map<(usize, usize), NodeIndex>,
    heads: VecDeque<NodeIndex>,
    following: Vec<NodeIndex>,
    index: usize
}

//> MACHINE -> DEFAULT
impl Default for Machine {
    fn default() -> Self {
        let mut graph = Graph::default();
        let node = graph.add_node(0);
        return Self {
            graph: graph,
            states: Map::from([((0, 0), node)]),
            heads: VecDeque::from([node]),
            following: Vec::new(),
            index: 0
        }
    }
}

//> MACHINE -> IMPLEMENTATION
impl Machine {
    fn shift(
        &mut self, 
        state: NodeIndex, 
        goto: &'static usize
    ) -> () {
        let to = *self.states.entry((*goto, self.index + 1)).or_insert_with(|| {
            let to = self.graph.add_node(*goto);
            self.following.push(to);
            to
        });
        self.graph.update_edge(state, to, ());
    }
    fn reduce(
        &mut self, 
        state: NodeIndex, 
        length: &'static usize,
        rule: &'static Rule
    ) -> () {
        let mut frontier = Vec::from([state]);
        for _ in 0..*length {
            for node in take(&mut frontier) {
                for edge in self.graph.edges_directed(node, Incoming) {
                    frontier.push(edge.source());
                }
            }
        }
        for from in frontier {
            let goto = GOTO[self.graph[from]][rule];
            let to = *self.states.entry((goto, self.index)).or_insert_with(|| {
                let to = self.graph.add_node(goto);
                self.heads.push_back(to);
                to
            });
            self.graph.update_edge(from, to, ());
        }
    }
    pub fn finish(self) -> () {}
    pub fn advance(&mut self) -> () {
        self.heads.extend(self.following.drain(..));
        self.index += 1;
    }
    pub fn pass(&mut self, token: &Token) -> () {
        let name = token.as_ref();
        while let Some(state) = self.heads.pop_front() {for action in ACTION[
            self.graph[state]
        ].get(name).map(Array::as_ref).unwrap_or_default() {match action {
            Action::Reduce {rule, length} => self.reduce(state, length, rule),
            Action::Shift {goto} => self.shift(state, goto),
        }}}
    }
}