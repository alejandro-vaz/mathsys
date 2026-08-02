//^
//^ HEAD
//^

//> HEAD -> PETGRAPH
use petgraph::Graph;

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
    trace::Trace,
    parse::Parse
};


//^
//^ STACK
//^

//> STACK -> STRUCT
pub struct Stack {
    start: Trace,
    accepted: Set<Trace>,
    traces: Map<(usize, Trace, Trace, State, &'static Action), Trace>,
    graph: Graph<usize, ()>,
    links: Map<Head, Set<Head>>,
    states: Map<(usize, usize), State>,
    processed: Set<Head>,
    heads: VecDeque<Head>,
    following: Set<Head>,
    path: Graph<&'static Action, ()>
}

//> STACK -> DEFAULT
impl Default for Stack {
    fn default() -> Self {
        static START: Action = Action::Start;
        let mut graph = Graph::default();
        let mut path = Graph::default();
        let state = State(graph.add_node(0));
        let trace = Trace(path.add_node(&START));
        let head = Head {
            state: state,
            trace: trace
        };
        return Self {
            start: trace,
            accepted: Set::new(),
            traces: Map::from([((0, trace, trace, state, &START), trace)]),
            graph: graph,
            links: Map::new(),
            states: Map::from([((0, 0), state)]),
            processed: Set::from([head]),
            heads: VecDeque::from([head]),
            following: Set::new(),
            path: path
        }
    }
}

//> STACK -> IMPLEMENTATION
impl<'valid> Stack {
    pub fn finish(self) -> Parse {return Parse {
        start: self.start,
        accepted: self.accepted,
        path: self.path
    }}
    pub fn get(&self, state: State) -> usize {return self.graph[state.0]}
    pub fn shift(&mut self, from: Head, to: Head) -> () {
        self.graph.update_edge(from.state.0, to.state.0, ());
        self.links.entry(to).or_default().insert(from);
        self.path.update_edge(from.trace.0, to.trace.0, ());
        self.following.insert(to);
    }
    pub fn reduce(&mut self, from: Head, with: Trace, to: Head) -> () {
        self.graph.update_edge(from.state.0, to.state.0, ());
        self.links.entry(to).or_default().insert(from);
        self.path.update_edge(with.0, to.trace.0, ());
        if self.processed.insert(to) {self.heads.push_back(to)}
    }
    pub fn accept(&mut self, from: Trace, with: Trace) -> () {
        self.path.update_edge(from.0, with.0, ());
        self.accepted.insert(with);
    }
    pub fn state(&mut self, rawstate: usize, index: usize) -> State {
        return *self.states.entry((rawstate, index)).or_insert_with(|| {
            State(self.graph.add_node(rawstate))
        });
    }
    pub fn trace(
        &mut self,
        action: &'static Action,
        index: usize,
        from: Trace,
        base: Trace,
        to: State
    ) -> Trace {return *self.traces.entry((index, from, base, to, action)).or_insert_with(|| {
        Trace(self.path.add_node(action))
    })}
    pub fn next(&mut self) -> Option<Head> {return self.heads.pop_front()}
    pub fn advance<'instance>(&'instance mut self) -> bool {
        self.heads.extend(self.following.drain());
        self.processed.clear();
        return !self.heads.is_empty();
    }
    pub fn frontier(&self, head: Head, length: usize) -> Set<Head> {
        let mut frontier = Set::from([head]);
        for _ in 0..length {for head in take(&mut frontier) {
            if let Some(predecessors) = self.links.get(&head) {
                frontier.extend(predecessors.iter().copied());
            }
        }}
        return frontier;
    }
}
