//^
//^ HEAD
//^

//> HEAD -> MODULES
mod extensor;
pub(super) mod grammar;
pub(super) mod state;

//> HEAD -> PRELUDE
use crate::prelude::{
    Map, 
    Set, 
    Deque, 
    SmallVec,
    Arena
};

//> HEAD -> LOCAL
use self::{
    grammar::{
        GRAMMAR, 
        Rule, 
        Symbol,
        Part
    },
    state::{
        State,
        Backpointer
    },
    super::{
        Settings,
        tokenizer::token::{
            BindedToken, 
            ORDER, 
            Responsibility
        }
    }
};


//^
//^ PARSER
//^

//> PARSER -> MINPOINTERS
pub(super) static MINPOINTERS: usize = 2;

//> PARSER -> STRUCT
pub(super) struct Parser {} impl Parser {
    pub(super) const fn new() -> Self {return Parser {}}
    pub(super) fn run<'parsing>(
        &self, 
        mut tokens: Vec<BindedToken<'parsing>>, 
        settings: &Settings
    ) -> Map<Backpointer<'parsing>, Set<SmallVec<[Backpointer<'parsing>; MINPOINTERS]>>> {
        tokens.retain(|token| if let Responsibility::Null = ORDER.get(&token.kind).unwrap().1 {false} else {true});
        let arena = Arena::new();
        let land = Arena::new();
        let mut chart = Vec::<Map<&State, Set<SmallVec<[&Backpointer<'parsing>; MINPOINTERS]>>>>::with_capacity(tokens.len() + 1);
        let mut waiting = Vec::<Map<&Symbol, Set<&State>>>::with_capacity(tokens.len() + 1);
        chart.resize_with(tokens.len() + 1, Map::new);
        waiting.resize_with(tokens.len() + 1, Map::new);
        let root = State::new(&arena, Rule::Internal(0), &GRAMMAR.get(&Rule::Internal(0)).unwrap()[0], 0, 0);
        chart[0].entry(root).or_default().insert(SmallVec::new());
        waiting[0].entry(root.at().unwrap()).or_default().insert(root);
        let mut pool = Map::new();
        let mut agenda = Deque::<&State>::new();
        for index in 0..(tokens.len() + 1) {
            let token = tokens.get(index);
            let mut enqueued = Set::<&State>::new();
            let mut seen = Set::<&State>::new();
            let mut completed_here = Map::<Symbol, Set<&State>>::new();
            for state in chart[index].keys().copied() {
                self.enqueue(state, &mut agenda, &mut enqueued, &seen);
            }
            while let Some(state) = agenda.pop_front() {
                enqueued.remove(&state);
                if !seen.insert(state) {continue}
                if state.at().is_none() {
                    self.complete(index, state, &mut agenda, &mut enqueued, &seen, &mut completed_here, &mut pool, &mut waiting, &mut chart, &arena, &land);
                } else if let (Some(Symbol::Kind(kind)), Some(value)) = (state.at(), token) && *kind == value.kind {
                    self.scan(index, state, value, &mut waiting, &mut chart, &arena, &land);
                } else {
                    self.predict(index, state, &mut agenda, &mut enqueued, &seen, &completed_here, &mut pool, &mut waiting, &mut chart, &arena, &land);
                }
            }
        };
        return pool.into_iter().map(|(backpointer, set)| (backpointer.clone(), set.into_iter().flat_map(|(index, state)| chart[index].get(state).into_iter().flat_map(|set| set.iter()).map(|small| small.iter().map(|pointer| (*pointer).clone()).collect::<SmallVec<[Backpointer<'parsing>; MINPOINTERS]>>()).collect::<Vec<_>>()).collect())).collect();
    }
    #[inline(always)]
    fn enqueue<'arena>(
        &self,
        state: &'arena State,
        agenda: &mut Deque<&'arena State>,
        enqueued: &mut Set<&'arena State>,
        seen: &Set<&'arena State>
    ) -> () {
        if !seen.contains(&state) && enqueued.insert(state) {
            agenda.push_back(state);
        }
    }
    #[inline(always)]
    fn complete<'parsing, 'arena, 'land>(
        &self, 
        index: usize, 
        state: &'arena State, 
        agenda: &mut Deque<&'arena State>, 
        enqueued: &mut Set<&'arena State>,
        seen: &Set<&'arena State>,
        completed_here: &mut Map<Symbol, Set<&'arena State>>,
        pool: &mut Map<&'land Backpointer<'parsing>, Set<(usize, &'arena State)>>,
        waiting: &mut Vec<Map<&Symbol, Set<&'arena State>>>, 
        chart: &mut Vec<Map<&'arena State, Set<SmallVec<[&'land Backpointer<'parsing>; MINPOINTERS]>>>>,
        arena: &'arena Arena<State>,
        land: &'land Arena<Backpointer<'parsing>>
    ) -> () {
        let symbol = Into::<Symbol>::into(state.rule);
        if let Some(waitlist) = waiting[state.starting].get(&symbol).cloned() {
            for awaiting in waitlist {
                self.advance(index, awaiting, state, agenda, enqueued, seen, completed_here, pool, waiting, chart, arena, land);
            }
        };
        if state.starting == index {
            completed_here.entry(symbol).or_default().insert(state);
        }
    }
    #[inline(always)]
    fn scan<'parsing, 'arena, 'land>(
        &self, 
        index: usize, 
        state: &'arena State, 
        token: &BindedToken<'parsing>, 
        waiting: &mut Vec<Map<&Symbol, Set<&'arena State>>>, 
        chart: &mut Vec<Map<&'arena State, Set<SmallVec<[&'land Backpointer<'parsing>; MINPOINTERS]>>>>,
        arena: &'arena Arena<State>,
        land: &'land Arena<Backpointer<'parsing>>
    ) -> () {
        let addable = chart[index].entry(state).or_default().clone().into_iter().map(|mut element| {element.push(Backpointer::new(land, Part::Token(token.clone()), index, index + 1)); element}).collect::<Vec<SmallVec<[&Backpointer; MINPOINTERS]>>>();
        let following = state.next(arena);
        chart[index + 1].entry(following).or_default().extend(addable);
        if let Some(symbol) = following.at() {waiting[index + 1].entry(symbol).or_default().insert(following);}
    }
    #[inline(always)]
    fn predict<'parsing, 'arena, 'land>(
        &self, 
        index: usize, 
        state: &'arena State, 
        agenda: &mut Deque<&'arena State>, 
        enqueued: &mut Set<&'arena State>,
        seen: &Set<&'arena State>,
        completed_here: &Map<Symbol, Set<&'arena State>>,
        pool: &mut Map<&'land Backpointer<'parsing>, Set<(usize, &'arena State)>>,
        waiting: &mut Vec<Map<&Symbol, Set<&'arena State>>>, 
        chart: &mut Vec<Map<&'arena State, Set<SmallVec<[&'land Backpointer<'parsing>; MINPOINTERS]>>>>,
        arena: &'arena Arena<State>,
        land: &'land Arena<Backpointer<'parsing>>
    ) -> () {
        let rule = match *state.at().unwrap() {
            Symbol::Internal(code) => Rule::Internal(code),
            Symbol::NonTerminal(object) => Rule::NonTerminal(object), 
            Symbol::Kind(kind) => return
        };
        for variant in &GRAMMAR[&rule] {
            let possibility = State::new(arena, rule, variant, 0, index);
            let produced = chart[index].entry(possibility).or_default();
            if produced.is_empty() {
                produced.insert(SmallVec::new());
                self.enqueue(possibility, agenda, enqueued, seen);
            }
            if let Some(symbol) = possibility.at() {
                if waiting[index].entry(symbol).or_default().insert(possibility) {
                    self.replay(index, possibility, symbol, agenda, enqueued, seen, completed_here, pool, waiting, chart, arena, land);
                }
            }
        }
    }
    #[inline(always)]
    fn replay<'parsing, 'arena, 'land>(
        &self,
        index: usize,
        awaiting: &'arena State,
        symbol: &'static Symbol,
        agenda: &mut Deque<&'arena State>,
        enqueued: &mut Set<&'arena State>,
        seen: &Set<&'arena State>,
        completed_here: &Map<Symbol, Set<&'arena State>>,
        pool: &mut Map<&'land Backpointer<'parsing>, Set<(usize, &'arena State)>>,
        waiting: &mut Vec<Map<&Symbol, Set<&'arena State>>>,
        chart: &mut Vec<Map<&'arena State, Set<SmallVec<[&'land Backpointer<'parsing>; MINPOINTERS]>>>>,
        arena: &'arena Arena<State>,
        land: &'land Arena<Backpointer<'parsing>>
    ) -> () {
        if let Some(completed_states) = completed_here.get(symbol).cloned() {
            for completed in completed_states {
                self.advance(index, awaiting, completed, agenda, enqueued, seen, completed_here, pool, waiting, chart, arena, land);
            }
        }
    }
    #[inline(always)]
    fn advance<'parsing, 'arena, 'land>(
        &self,
        index: usize,
        awaiting: &'arena State,
        completed: &'arena State,
        agenda: &mut Deque<&'arena State>,
        enqueued: &mut Set<&'arena State>,
        seen: &Set<&'arena State>,
        completed_here: &Map<Symbol, Set<&'arena State>>,
        pool: &mut Map<&'land Backpointer<'parsing>, Set<(usize, &'arena State)>>,
        waiting: &mut Vec<Map<&Symbol, Set<&'arena State>>>,
        chart: &mut Vec<Map<&'arena State, Set<SmallVec<[&'land Backpointer<'parsing>; MINPOINTERS]>>>>,
        arena: &'arena Arena<State>,
        land: &'land Arena<Backpointer<'parsing>>
    ) -> () {
        let advanced = awaiting.next(arena);
        let pointer = Backpointer::new(land, Into::<Part>::into(completed.rule), completed.starting, index);
        pool.entry(pointer).or_default().insert((index, completed));
        let backpointers = if let Some(previous) = chart[completed.starting].get(awaiting) {previous.iter().map(|backpointer| {
            let mut new = backpointer.clone();
            new.push(pointer);
            new 
        }).collect()} else {Vec::new()};
        let additional = chart[index].entry(advanced).or_default();
        if additional.is_empty() {
            additional.extend(backpointers);
            self.enqueue(advanced, agenda, enqueued, seen);
        } else if let Rule::NonTerminal(_) = advanced.rule {
            additional.extend(backpointers);
        }
        if let Some(symbol) = advanced.at() {
            if waiting[index].entry(symbol).or_default().insert(advanced) {
                self.replay(index, advanced, symbol, agenda, enqueued, seen, completed_here, pool, waiting, chart, arena, land);
            }
        }
    }
}
