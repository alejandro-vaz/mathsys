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
        let mut chart = Vec::<Map<&State, Set<SmallVec<[Backpointer<'parsing>; MINPOINTERS]>>>>::with_capacity(tokens.len() + 1);
        let mut waiting = Vec::<Map<&Symbol, Set<&State>>>::with_capacity(tokens.len() + 1);
        chart.resize_with(tokens.len() + 1, Map::new);
        waiting.resize_with(tokens.len() + 1, Map::new);
        let root = State::new(&arena, Rule::Internal(0), &GRAMMAR.get(&Rule::Internal(0)).unwrap()[0], 0, 0);
        chart[0].entry(root).or_default().insert(SmallVec::new());
        waiting[0].entry(root.at().unwrap()).or_default().insert(root);
        let mut pool = Map::new();
        let mut agenda = Deque::<&State>::new();
        let mut completed = Set::new();
        for index in 0..(tokens.len() + 1) {
            let token = tokens.get(index as usize);
            agenda.extend(chart[index as usize].keys());
            while !agenda.is_empty() {
                let state = agenda.pop_front().unwrap();
                if state.at().is_none() {
                    self.complete(index, state, &mut agenda, &mut completed, &mut pool, &mut waiting, &mut chart, &arena);
                } else if let (Some(Symbol::Kind(kind)), Some(value)) = (state.at(), token) && *kind == value.kind {
                    self.scan(index, state, value, &mut waiting, &mut chart, &arena);
                } else {
                    self.predict(index, state, &mut agenda, &mut waiting, &mut chart, &arena);
                }
            }
            completed.clear();
        };
        return pool.into_iter().map(|(backpointer, set)| (backpointer, set.into_iter().flat_map(|(index, state)| chart[index as usize].entry(state).or_default().clone().into_iter()).collect())).collect();
    }
    #[inline(always)]
    fn complete<'parsing, 'arena>(
        &self, 
        index: usize, 
        state: &'arena State, 
        agenda: &mut Deque<&'arena State>, 
        completed: &mut Set<&'arena State>, 
        pool: &mut Map<Backpointer<'parsing>, Set<(usize, &'arena State)>>,
        waiting: &mut Vec<Map<&Symbol, Set<&'arena State>>>, 
        chart: &mut Vec<Map<&'arena State, Set<SmallVec<[Backpointer<'parsing>; MINPOINTERS]>>>>,
        arena: &'arena Arena<State>
    ) -> () {
        if let Some(waitlist) = waiting[state.starting as usize].get(&state.rule.into() as &Symbol).cloned() {for awaiting in waitlist {
            let advanced = awaiting.next(arena);
            let pointer = Backpointer::new(state.rule.into(), state.starting, index);
            pool.entry(pointer.clone()).or_default().insert((index, state));
            let backpointers = chart[state.starting as usize].get(awaiting).cloned().unwrap_or_default().into_iter().map(|mut backpointer| {backpointer.push(pointer.clone()); backpointer}).collect::<Vec<SmallVec<[Backpointer; MINPOINTERS]>>>();
            if let Some(symbol) = advanced.at() {waiting[index as usize].entry(symbol).or_default().insert(advanced);}
            let additional = chart[index as usize].entry(advanced).or_default();
            if additional.is_empty() {
                additional.extend(backpointers);
                agenda.push_back(advanced);
                agenda.extend(completed.iter());
                completed.clear();
            }
        }}
        completed.insert(state);
    }
    #[inline(always)]
    fn scan<'parsing, 'arena>(
        &self, 
        index: usize, 
        state: &'arena State, 
        token: &BindedToken<'parsing>, 
        waiting: &mut Vec<Map<&Symbol, Set<&'arena State>>>, 
        chart: &mut Vec<Map<&'arena State, Set<SmallVec<[Backpointer<'parsing>; MINPOINTERS]>>>>,
        arena: &'arena Arena<State>
    ) -> () {
        let addable = chart[index as usize].entry(state).or_default().clone().into_iter().map(|mut element| {element.push(Backpointer::new(Part::Token(token.clone()), index, index + 1)); element}).collect::<Vec<SmallVec<[Backpointer; MINPOINTERS]>>>();
        let following = state.next(arena);
        chart[index as usize + 1].entry(following).or_default().extend(addable);
        if let Some(symbol) = following.at() {waiting[index as usize + 1].entry(symbol).or_default().insert(following);}
    }
    #[inline(always)]
    fn predict<'arena>(
        &self, 
        index: usize, 
        state: &'arena State, 
        agenda: &mut Deque<&'arena State>, 
        waiting: &mut Vec<Map<&Symbol, Set<&'arena State>>>, 
        chart: &mut Vec<Map<&'arena State, Set<SmallVec<[Backpointer; MINPOINTERS]>>>>,
        arena: &'arena Arena<State>
    ) -> () {
        let rule = match *state.at().unwrap() {
            Symbol::Internal(code) => Rule::Internal(code),
            Symbol::NonTerminal(object) => Rule::NonTerminal(object), 
            Symbol::Kind(kind) => return
        };
        for variant in &GRAMMAR[&rule] {
            let possibility = State::new(arena, rule, variant, 0, index);
            if let Some(symbol) = possibility.at() {waiting[index as usize].entry(symbol).or_default().insert(possibility);}
            let produced = chart[index as usize].entry(possibility).or_default();
            if produced.is_empty() {
                produced.insert(SmallVec::new());
                agenda.push_back(possibility);
            }
        }
    }
}