//^
//^ HEAD
//^

//> HEAD -> PETGRAPH
use petgraph::{
    Graph,
    Directed,
    graph::NodeIndex,
    Incoming
};

//> HEAD -> STD
use std::collections::{
    HashMap as Map,
    VecDeque,
    HashSet as Set
};


//^
//^ STACK
//^

//> STACK -> STRUCT
pub struct Stack {
    graph: Graph<usize, (), Directed>,
    states: Map<(usize, usize), NodeIndex>,
    processed: Set<NodeIndex>,
    heads: VecDeque<NodeIndex>,
    following: Set<NodeIndex>
}

//> STACK -> DEFAULT
impl Default for Stack {
    fn default() -> Self {
        let mut graph = Graph::default();
        let node = graph.add_node(0);
        return Self {
            graph: graph,
            states: Map::from([((0, 0), node)]),
            processed: Set::from([node]),
            heads: VecDeque::from([node]),
            following: Set::new(),
        }
    }
}

//> STACK -> IMPLEMENTATION
impl<'valid> Stack {
    pub fn get(&self, node: NodeIndex) -> usize {return self.graph[node]}
    pub fn shift(&mut self, from: NodeIndex, to: NodeIndex) -> () {
        self.graph.update_edge(from, to, ());
        self.following.insert(to);
    }
    pub fn reduce(&mut self, from: NodeIndex, to: NodeIndex) -> () {
        self.graph.update_edge(from, to, ());
        if self.processed.insert(to) {self.heads.push_back(to)}
    }
    pub fn accept(&mut self) -> () {}
    pub fn state(&mut self, rawstate: usize, index: usize) -> NodeIndex {
        return *self.states.entry((rawstate, index)).or_insert_with(|| {
            self.graph.add_node(rawstate)
        });
    }
    pub fn next(&mut self) -> Option<NodeIndex> {return self.heads.pop_front()}
    pub fn advance<'instance>(&'instance mut self) -> bool {
        self.heads.extend(self.following.drain());
        self.processed.clear();
        return !self.heads.is_empty();
    }
    pub fn frontier(&self, state: NodeIndex, length: usize) -> Set<NodeIndex> {
        let mut frontier = Set::from([state]);
        for _ in 0..length {for node in frontier.drain().collect::<Vec<_>>() {
            frontier.extend(self.graph.neighbors_directed(node, Incoming));
        }}
        return frontier;
    }
}
