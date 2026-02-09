//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    FastMap, FastSet, Deque, SmallVec
};

//> HEAD -> LOCAL
use super::super::Settings;
use super::tokenizer::{BindedToken, ORDER, Responsibility, Kind};
use super::grammar::{GRAMMAR, Rule, Symbol};


//^
//^ RESOURCES
//^

//> RESOURCES -> STATE
#[derive(Clone, Hash, PartialEq, Eq)]
struct State {
    rule: Rule,
    variant: u8,
    slot: u8,
    starting: u32
} impl State {
    #[inline(always)]
    pub fn at(&self) -> Option<Symbol> {
        let productions = &GRAMMAR[&self.rule];
        if self.slot as usize == productions[self.variant as usize].len() {None} else {Some(productions[self.variant as usize][self.slot as usize].clone())}
    }
    #[inline(always)]
    pub fn next(&self) -> State {return State {
        rule: self.rule,
        variant: self.variant,
        slot: self.slot + 1,
        starting: self.starting
    }}
}

//> RESOURCES -> BACKPOINTER
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Backpointer {
    pub symbol: Symbol,
    pub start: u32,
    pub end: u32
}

//^
//^ PARSER
//^

//> PARSER -> MINPOINTERS
pub static MINPOINTERS: usize = 2;

//> PARSER -> STRUCT
pub struct Recognizer {
    chart: Vec<FastMap<State, FastSet<SmallVec<[Backpointer; MINPOINTERS]>>>> = Vec::new(),
    waiting: Vec<FastMap<Symbol, FastSet<State>>> = Vec::new()
} impl Recognizer {
    pub fn new() -> Self {return Recognizer {..}}
    #[inline(always)]
    fn wait(&mut self, index: u32, state: &State) -> () {if let Some(symbol) = state.at() {self.waiting[index as usize].entry(symbol).or_default().insert(state.clone());}}
    #[inline(always)]
    fn recall(&mut self, index: u32, state: &State) -> &mut FastSet<SmallVec<[Backpointer; MINPOINTERS]>> {return self.chart[index as usize].entry(state.clone()).or_default()}
    #[inline(always)]
    fn review(&mut self, index: u32, state: &State) -> &FastSet<SmallVec<[Backpointer; MINPOINTERS]>> {return self.chart[index as usize].entry(state.clone()).or_default()}
    pub fn run(&mut self, tokens: &Vec<BindedToken>, settings: &Settings) -> FastMap<Backpointer, FastSet<SmallVec<[Backpointer; MINPOINTERS]>>> {
        let mut filtered = tokens.clone();
        filtered.retain(|token| if let Responsibility::Null = ORDER.get(&token.kind).unwrap().1 {false} else {true});
        self.chart.clear();
        self.waiting.clear();
        self.chart.extend((0..(filtered.len() + 1)).map(|iteration| FastMap::new()));
        self.waiting.extend((0..(filtered.len() + 1)).map(|iteration| FastMap::new()));
        let root = State {
            rule: Rule::Internal(0),
            variant: 0,
            slot: 0,
            starting: 0
        };
        self.recall(0, &root).insert(SmallVec::new());
        self.wait(0, &root);
        let mut pool = FastMap::new() as FastMap<Backpointer, FastSet<SmallVec<[Backpointer; MINPOINTERS]>>>;
        let mut agenda = Deque::new();
        let mut completed = FastSet::new();
        for index in 0..(filtered.len() + 1) {
            let token = filtered.get(index as usize);
            agenda.extend(self.chart[index as usize].keys().cloned());
            while !agenda.is_empty() {
                let state = agenda.pop_front().unwrap();
                if state.at().is_none() {
                    self.complete(index as u32, state, &mut agenda, &mut completed, &mut pool);
                } else if let (Some(Symbol::Kind(kind)), Some(value)) = (state.at(), token) && kind == value.kind {
                    self.scan(index as u32, state, &mut agenda, kind);
                } else {
                    self.predict(index as u32, state, &mut agenda);
                }
            }
            completed.clear();
        };
        return pool;
    }
    #[inline(always)]
    fn complete(&mut self, index: u32, state: State, agenda: &mut Deque<State>, completed: &mut FastSet<State>, pool: &mut FastMap<Backpointer, FastSet<SmallVec<[Backpointer; MINPOINTERS]>>>) -> () {
        for awaiting in self.waiting[state.starting as usize].get(&state.rule.into()).cloned().unwrap_or_default() {
            let advanced = awaiting.next();
            let stored = self.review(index, &advanced).len();
            let pointer = Backpointer {
                symbol: state.rule.into(),
                start: state.starting,
                end: index
            };
            pool.entry(pointer.clone()).or_default().extend(self.review(index, &state).clone());
            let backpointers = self.chart[state.starting as usize].get(&awaiting).cloned().unwrap_or_default().into_iter().map(|mut backpointer| {backpointer.push(pointer.clone()); backpointer}).collect::<Vec<SmallVec<[Backpointer; MINPOINTERS]>>>();
            self.wait(index, &advanced);
            let additional = self.recall(index, &advanced);
            additional.extend(backpointers);
            if additional.len() > stored {
                agenda.push_back(advanced);
                agenda.extend(completed.clone());
            }
        }
        completed.insert(state);
    }
    #[inline(always)]
    fn scan(&mut self, index: u32, state: State, agenda: &mut Deque<State>, kind: Kind) -> () {
        let addable = self.recall(index, &state).clone().into_iter().map(|mut element| {element.push(Backpointer {
            symbol: Symbol::Kind(kind), 
            start: index, 
            end: index + 1
        }); element}).collect::<Vec<SmallVec<[Backpointer; MINPOINTERS]>>>();
        let following = state.next();
        self.recall(index + 1, &following).extend(addable);
        self.wait(index + 1, &following);
    }
    #[inline(always)]
    fn predict(&mut self, index: u32, state: State, agenda: &mut Deque<State>) -> () {
        let rule = match state.at().unwrap() {
            Symbol::Internal(code) => Rule::Internal(code),
            Symbol::NonTerminal(object) => Rule::NonTerminal(object), 
            Symbol::Kind(kind) => return
        };
        for variant in 0..GRAMMAR[&rule].len() {
            let possibility = State {
                rule: rule,
                variant: variant as u8,
                slot: 0,
                starting: index
            };
            self.wait(index, &possibility);
            let produced = self.recall(index, &possibility);
            if produced.is_empty() {
                produced.insert(SmallVec::new());
                agenda.push_back(possibility);
            }
        }
    }
}