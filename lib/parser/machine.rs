//^
//^ HEAD
//^

//> HEAD -> PETGRAPH
use petgraph::{
    Graph,
    graph::NodeIndex,
    Incoming
};

//> HEAD -> STD
use std::collections::{
    HashMap as Map, 
    HashSet as Set, 
    VecDeque
};

//> HEAD -> SUPER
use super::{
    rule::Rule,
    forest::Forest,
    tables::GOTO
};

//> HEAD -> CRATE
use crate::tokenizer::token::Token;


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
    pub fn finish(self) -> Forest<'valid> {return self.forest}
    pub fn get(&self, node: NodeIndex) -> usize {return self.graph[node]}
    pub fn shift(
        &mut self, 
        from: NodeIndex, 
        token: &'valid Token<'valid>,
        goto: &'static usize
    ) -> () {
        let to = *self.states.entry((*goto, self.index + 1)).or_insert_with(|| {
            self.graph.add_node(*goto)
        });
        let node = self.forest.shift(token, self.index);
        self.graph.update_edge(from, to, node);
        self.following.insert(to);
    }
    pub fn reduce(
        &mut self, 
        state: NodeIndex, 
        length: &'static usize,
        rule: &'static Rule
    ) -> () {
        let mut frontier = Set::from([state]);
        let vector = Vec::new();
        for _ in 0..*length {
            vector.extend(frontier.drain());
            for node in vector.drain(..) {
                frontier.extend(self.graph.neighbors_directed(node, Incoming));
            }
        }
        for from in frontier {
            let goto = GOTO[self.get(from)][rule];
            let to = *self.states.entry((goto, self.index)).or_insert_with(|| {
                self.graph.add_node(goto)
            });
            self.graph.update_edge(from, to, ());
            if self.processed.insert(to) {self.heads.push_back(to)}
        }
    }
    pub fn accept(&mut self, state: NodeIndex) -> () {
        let edges = self.graph.edges_directed(state, Incoming);
        self.forest.accept(edges);
    }
    pub fn next(&mut self) -> Option<NodeIndex> {return self.heads.pop_front()}
    pub fn advance(&mut self) -> () {
        self.heads.extend(self.following.drain());
        self.processed.clear();
        self.index += 1;
    }
}