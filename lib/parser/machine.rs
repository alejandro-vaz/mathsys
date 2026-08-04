//^
//^ HEAD
//^

//> HEAD -> PETGRAPH
use petgraph::{
    Graph,
    graph::NodeIndex,
    visit::EdgeRef,
    Direction::{
        Incoming,
        Outgoing
    }
};

//> HEAD -> STD
use std::collections::{
    HashMap as Map,
    HashSet as Set,
    VecDeque
};

//> HEAD -> SUPER
use super::{
    action::Action,
    forest::{
        Forest,
        NodeId,
        ProductionId
    },
    production::Production,
    rule::Rule,
    symbol::Symbol,
    tables::{
        ACTION,
        GOTO
    }
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
pub struct Machine<'input> {
    graph: Graph<State, NodeId>,
    states: Map<(usize, usize), NodeIndex>,
    heads: VecDeque<NodeIndex>,
    following: Vec<NodeIndex>,
    index: usize,
    forest: Forest<'input>
}

//> MACHINE -> STATE
#[derive(Clone, Copy)]
struct State {
    number: usize,
    index: usize
}

//> MACHINE -> DEFAULT
impl<'input> Default for Machine<'input> {
    fn default() -> Self {
        let mut graph = Graph::default();
        let node = graph.add_node(State {number: 0, index: 0});
        return Self {
            graph: graph,
            states: Map::from([((0, 0), node)]),
            heads: VecDeque::from([node]),
            following: Vec::new(),
            index: 0,
            forest: Forest::default()
        }
    }
}

//> MACHINE -> IMPLEMENTATION
impl<'input> Machine<'input> {
    fn shift(
        &mut self,
        state: NodeIndex,
        goto: usize,
        terminal: NodeId
    ) -> () {
        let (to, created) = self.state(goto, self.index + 1);
        if created {self.following.push(to)}
        self.link(state, to, terminal);
    }
    fn reduce(
        &mut self,
        state: NodeIndex,
        production: &Production
    ) -> Vec<NodeIndex> {
        let production_id = ProductionId::new(production);
        let mut destinations = Vec::new();
        let length = production.derivation.len();
        if length == 0 {
            self.complete(
                state,
                state,
                production,
                production_id,
                None,
                None,
                self.index,
                &mut destinations
            );
            return destinations;
        }
        let incoming = self.incoming(state);
        if length == 1 {
            let mut matching = Vec::new();
            for (from, child) in incoming {
                if self.matches(child, &production.derivation[0]) {
                    matching.push((from, child));
                }
            }
            for (from, child) in matching {
                let pivot = self.graph[from].index;
                self.complete(
                    from,
                    state,
                    production,
                    production_id,
                    None,
                    Some(child),
                    pivot,
                    &mut destinations
                );
            }
            return destinations;
        }
        let mut memo = Map::new();
        let mut matching = Vec::new();
        for (before_last, last) in incoming {
            if self.matches(last, &production.derivation[length - 1]) {
                matching.push((before_last, last));
            }
        }
        for (before_last, last) in matching {
            let prefixes = self.prefixes(
                before_last,
                length - 1,
                production,
                &mut memo
            );
            for (from, prefix) in prefixes {
                self.complete(
                    from,
                    state,
                    production,
                    production_id,
                    Some(prefix),
                    Some(last),
                    self.graph[before_last].index,
                    &mut destinations
                );
            }
        }
        return destinations;
    }
    fn complete(
        &mut self,
        from: NodeIndex,
        state: NodeIndex,
        production: &Production,
        production_id: ProductionId,
        left: Option<NodeId>,
        right: Option<NodeId>,
        pivot: usize,
        destinations: &mut Vec<NodeIndex>
    ) -> () {
        let left_extent = self.graph[from].index;
        let nonterminal = self.forest.nonterminal(
            production.rule,
            left_extent,
            self.graph[state].index
        );
        self.forest.add_family(
            nonterminal,
            production_id,
            pivot,
            left,
            right
        );
        if *production.rule == Rule::default() && left_extent == 0 {
            self.forest.add_root(nonterminal);
        }
        let Some(&goto) = GOTO[self.graph[from].number].get(production.rule) else {
            return;
        };
        let (to, _) = self.state(goto, self.index);
        if self.link(from, to, nonterminal) {destinations.push(to)}
    }
    fn prefixes(
        &mut self,
        state: NodeIndex,
        length: usize,
        production: &Production,
        memo: &mut Map<(NodeIndex, usize), Map<NodeIndex, NodeId>>
    ) -> Map<NodeIndex, NodeId> {
        if let Some(prefixes) = memo.get(&(state, length)) {return prefixes.clone()}
        let mut prefixes = Map::new();
        let mut matching = Vec::new();
        for (previous, child) in self.incoming(state) {
            if self.matches(child, &production.derivation[length - 1]) {
                matching.push((previous, child));
            }
        }
        for (previous, child) in matching {
            if length == 1 {
                prefixes.insert(previous, child);
                continue;
            }
            for (from, prefix) in self.prefixes(
                previous,
                length - 1,
                production,
                memo
            ) {
                let intermediate = self.forest.intermediate(
                    ProductionId::new(production),
                    length,
                    self.graph[from].index,
                    self.graph[state].index
                );
                self.forest.add_family(
                    intermediate,
                    ProductionId::new(production),
                    self.graph[previous].index,
                    Some(prefix),
                    Some(child)
                );
                prefixes.insert(from, intermediate);
            }
        }
        memo.insert((state, length), prefixes.clone());
        return prefixes;
    }
    fn incoming(&self, state: NodeIndex) -> Vec<(NodeIndex, NodeId)> {
        return self.graph.edges_directed(state, Incoming).map(|edge| {
            (edge.source(), *edge.weight())
        }).collect();
    }
    fn matches(&self, node: NodeId, symbol: &Symbol) -> bool {
        return match (self.forest.node(node).label(), symbol) {
            (super::forest::NodeLabel::Terminal(token), Symbol::str(name)) => {
                token.as_ref() == *name
            },
            (super::forest::NodeLabel::EndOfFile, Symbol::str(name)) => {
                *name == "ENDOFFILE"
            },
            (super::forest::NodeLabel::Nonterminal(rule), Symbol::Rule(expected)) => {
                rule == expected
            },
            _ => false
        };
    }
    fn state(&mut self, number: usize, index: usize) -> (NodeIndex, bool) {
        if let Some(&state) = self.states.get(&(number, index)) {
            return (state, false);
        }
        let state = self.graph.add_node(State {number, index});
        self.states.insert((number, index), state);
        return (state, true);
    }
    fn link(&mut self, from: NodeIndex, to: NodeIndex, label: NodeId) -> bool {
        if self.graph.edges_connecting(from, to).any(|edge| {
            *edge.weight() == label
        }) {return false}
        self.graph.add_edge(from, to, label);
        return true;
    }
    fn schedule(
        &self,
        state: NodeIndex,
        queue: &mut VecDeque<NodeIndex>,
        queued: &mut Set<NodeIndex>
    ) -> () {
        let mut pending = Vec::from([state]);
        let mut visited = Set::new();
        while let Some(state) = pending.pop() {
            if !visited.insert(state) || self.graph[state].index != self.index {continue}
            if queued.insert(state) {queue.push_back(state)}
            for edge in self.graph.edges_directed(state, Outgoing) {
                pending.push(edge.target());
            }
        }
    }
    fn process(&mut self, token: &'input Token<'input>, consume: bool) -> () {
        let name = token.as_ref();
        let mut terminal = None;
        let mut queue = take(&mut self.heads);
        let mut queued = queue.iter().copied().collect::<Set<_>>();
        while let Some(state) = queue.pop_front() {
            queued.remove(&state);
            let actions = ACTION[self.graph[state].number].get(name).map(Array::as_ref);
            let Some(actions) = actions else {continue};
            for action in actions {match action {
                Action::Reduce {production} => {
                    for destination in self.reduce(state, production) {
                        self.schedule(destination, &mut queue, &mut queued);
                    }
                },
                Action::Shift {goto} if consume => {
                    let terminal = *terminal.get_or_insert_with(|| {
                        self.forest.terminal(token, self.index)
                    });
                    self.shift(state, *goto, terminal);
                },
                Action::Shift {..} => ()
            }}
        }
    }
    pub fn finish(mut self) -> Forest<'input> {
        self.process(&Token::EndOfFile, false);
        return self.forest;
    }
    pub fn advance(&mut self) -> () {
        self.heads.extend(self.following.drain(..));
        self.index += 1;
    }
    pub fn pass(&mut self, token: &'input Token<'input>) -> () {
        self.process(token, true);
    }
}
