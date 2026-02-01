//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    AHashMap, AHashSet, VecDeque, SmallVec
};

//> HEAD -> LOCAL
use super::tokenizer::{Token, ORDER, Responsibility};
use super::grammar::{GRAMMAR, Rule, Symbol};
use super::issues::{syntax, Issue};


//^
//^ RESOURCES
//^

//> RESOURCES -> STATE
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct State {
    rule: Rule,
    variant: u8,
    slot: u8,
    starting: u16
} impl State {
    #[inline(always)]
    pub fn new(rule: Rule, variant: u8, slot: u8, starting: u16) -> Self {return State {
        rule: rule,
        variant: variant,
        slot: slot,
        starting: starting
    }}
    #[inline(always)]
    pub fn full(&self) -> bool {self.slot == GRAMMAR.get(&self.rule).unwrap()[self.variant as usize].len() as u8}
    #[inline(always)]
    pub fn at(&self) -> Option<Symbol> {if self.full() {None} else {Some(GRAMMAR.get(&self.rule).unwrap()[self.variant as usize][self.slot as usize])}}
}

//> RESOURCES -> FOLLOW
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Follow {
    symbol: Symbol,
    start: u16,
    end: u16
} impl Follow {
    #[inline(always)]
    pub fn new(symbol: Symbol, start: u16, end: u16) -> Self {return Follow {
        symbol: symbol,
        start: start,
        end: end
    }}
}


//^
//^ PARSER
//^

//> PARSER -> SAVEDFOLLOWERS
static SAVEDFOLLOWERS: usize = 2;

//> PARSER -> STRUCT
pub struct Parser {
    chart: Vec<AHashMap<State, AHashSet<SmallVec<[Follow; SAVEDFOLLOWERS]>>>>,
    pool: AHashMap<Follow, AHashSet<SmallVec<[Follow; SAVEDFOLLOWERS]>>>,
    waiting: Vec<AHashMap<Symbol, AHashSet<State>>>
} impl Parser {
    pub fn new() -> Self {return Parser {
        chart: Vec::new(),
        pool: AHashMap::new(),
        waiting: Vec::new()
    }}
    #[inline(always)]
    fn wait(&mut self, index: u16, state: State) -> () {if let Some(symbol) = state.at() {self.waiting[index as usize].entry(symbol).or_default().insert(state);}}
    #[inline(always)]
    fn recall(&mut self, index: u16, state: State) -> &mut AHashSet<SmallVec<[Follow; SAVEDFOLLOWERS]>> {return self.chart[index as usize].entry(state).or_default()}
    #[inline(always)]
    fn review(&mut self, index: u16, state: State) -> &AHashSet<SmallVec<[Follow; SAVEDFOLLOWERS]>> {return self.chart[index as usize].entry(state).or_default()}
    fn reset(&mut self, mut tokens: Vec<Token>) -> Vec<Token> {
        tokens = tokens.into_iter().filter(|token| ORDER.get(&token.kind).unwrap().1 != Responsibility::Null).collect();
        self.chart.clear();
        self.pool.clear();
        self.waiting.clear();
        self.chart.extend((0..(tokens.len() + 1)).map(|iteration| AHashMap::new()));
        self.waiting.extend((0..(tokens.len() + 1)).map(|iteration| AHashMap::new()));
        let root = State::new(Rule::Internal(0), 0, 0, 0);
        self.recall(0, root).insert(SmallVec::new());
        self.wait(0, root);
        return tokens;
    }
    pub fn run(&mut self, mut tokens: Vec<Token>) -> Result<bool, Issue> {
        tokens = self.reset(tokens);
        let end = self.parse(tokens);
        return Ok(!end.is_empty());
    }
    fn parse(&mut self, tokens: Vec<Token>) -> &AHashSet<SmallVec<[Follow; SAVEDFOLLOWERS]>> {
        let length = tokens.len();
        let mut agenda = VecDeque::new();
        let mut completed = AHashSet::new();
        for index in (0 as u16)..((length + 1) as u16) {
            let token = tokens.get(index as usize);
            agenda.extend(self.chart[index as usize].keys().cloned());
            while !agenda.is_empty() {
                let state = agenda.pop_front().unwrap();
                if state.at().is_none() {
                    for awaiting in self.waiting[state.starting as usize].get(&state.rule.into()).cloned().unwrap_or_default() {
                        let advanced = State::new(awaiting.rule, awaiting.variant, awaiting.slot + 1, awaiting.starting);
                        let stored = self.review(index, advanced).len();
                        let pointer = Follow::new(state.rule.into(), state.starting, index);
                        let current = self.review(index, state).clone();
                        self.pool.entry(pointer).or_default().extend(current);
                        let backpointers = self.chart[state.starting as usize].get(&awaiting).cloned().unwrap_or_default().into_iter().map(|mut backpointer| {backpointer.push(pointer); backpointer}).collect::<Vec<SmallVec<[Follow; SAVEDFOLLOWERS]>>>();
                        self.wait(index, advanced);
                        let additional = self.recall(index, advanced);
                        additional.extend(backpointers);
                        if additional.len() > stored {
                            agenda.push_back(advanced);
                            agenda.extend(&completed);
                        }
                    }
                    completed.insert(state);
                } else if let (Some(Symbol::Kind(kind)), Some(value)) = (state.at(), token) && kind == value.kind {
                    let addable = self.recall(index, state).clone().into_iter().map(|mut element| {element.push(Follow::new(Symbol::Kind(kind), index, index + 1)); element}).collect::<Vec<SmallVec<[Follow; SAVEDFOLLOWERS]>>>();
                    let following = State::new(state.rule, state.variant, state.slot + 1, state.starting);
                    self.recall(index + 1, following).extend(addable);
                    self.wait(index + 1, following);
                } else {
                    let rule = match state.at().unwrap() {
                        Symbol::Internal(code) => Rule::Internal(code),
                        Symbol::NonTerminal(object) => Rule::NonTerminal(object), 
                        Symbol::Kind(kind) => continue
                    };
                    for variant in 0..GRAMMAR.get(&rule).unwrap().len() {
                        let possibility = State::new(rule, variant as u8, 0, index);
                        self.wait(index, possibility);
                        let produced = self.recall(index, possibility);
                        if produced.is_empty() {
                            produced.insert(SmallVec::new());
                            agenda.push_back(possibility);
                        }
                    }
                }
            }
            completed.clear();
        };
        return self.chart[length].entry(State::new(Rule::Internal(0), 0, 1, 0)).or_default();
    }
}