//^
//^ HEAD
//^

//> HEAD -> PETGRAPH
use petgraph::{
    Graph,
    Directed,
    Incoming
};

//> HEAD -> STD
use std::collections::{
    HashMap as Map,
    VecDeque,
    HashSet as Set
};

//> HEAD -> CORE
use core::mem::take;

//> HEAD -> SUPER
use super::{
    action::Action,
    head::Head,
    state::State,
    trace::Trace
};


//^
//^ STACK
//^

//> STACK -> STRUCT
#[derive(Debug)] // rm
pub struct Stack {
    traces: Map<(usize, &'static Action), Trace>,
    graph: Graph<usize, (), Directed>,
    states: Map<(usize, usize), State>,
    processed: Set<Head>,
    heads: VecDeque<Head>,
    following: Set<Head>,
    forest: Graph<&'static Action, (), Directed>
}

//> STACK -> DEFAULT
impl Default for Stack {
    fn default() -> Self {
        static START: &'static Action = &Action::Start;
        let mut graph = Graph::default();
        let mut forest = Graph::default();
        let state = State(graph.add_node(0));
        let trace = Trace(forest.add_node(START));
        let head = Head {
            state: state,
            trace: trace
        };
        return Self {
            traces: Map::from([((0, START), trace)]),
            graph: graph,
            states: Map::from([((0, 0), state)]),
            processed: Set::from([head]),
            heads: VecDeque::from([head]),
            following: Set::new(),
            forest: forest
        }
    }
}

//> STACK -> IMPLEMENTATION
impl<'valid> Stack {
    pub fn get(&self, state: State) -> usize {return self.graph[state.0]}
    pub fn shift(&mut self, from: Head, to: Head) -> () {
        self.graph.update_edge(from.state.0, to.state.0, ());
        self.forest.update_edge(from.trace.0, to.trace.0, ());
        self.following.insert(to);
    }
    pub fn reduce(&mut self, from: State, with: Trace, to: Head) -> () {
        self.graph.update_edge(from.0, to.state.0, ());
        self.forest.update_edge(with.0, to.trace.0, ());
        if self.processed.insert(to) {self.heads.push_back(to)}
    }
    pub fn state(&mut self, rawstate: usize, index: usize) -> State {
        return *self.states.entry((rawstate, index)).or_insert_with(|| {
            State(self.graph.add_node(rawstate))
        });
    }
    pub fn trace(&mut self, action: &'static Action, index: usize) -> Trace {
        return *self.traces.entry((index, action)).or_insert_with(|| {
            Trace(self.forest.add_node(action))
        })
    }
    pub fn next(&mut self) -> Option<Head> {return self.heads.pop_front()}
    pub fn advance<'instance>(&'instance mut self) -> bool {
        self.heads.extend(self.following.drain());
        self.processed.clear();
        return !self.heads.is_empty();
    }
    pub fn frontier(&self, state: State, length: usize) -> Set<State> {
        let mut frontier = Set::from([state]);
        for _ in 0..length {for node in take(&mut frontier) {
            frontier.extend(self.graph.neighbors_directed(node.0, Incoming).map(State));
        }}
        return frontier;
    }
}