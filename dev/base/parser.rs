//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    FastMap, FastSet, Deque, SmallVec
};

//> HEAD -> LOCAL
use super::super::Settings;
use super::tokenizer::{BindedToken, ORDER, Responsibility};
use super::grammar::{GRAMMAR, Rule, Symbol};
use super::issues::Issue;
use super::nonterminal::{Object, NonTerminal, Partition};
use super::start::Start;


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
    pub fn new(rule: Rule, variant: u8, slot: u8, starting: u32) -> Self {return State {
        rule: rule,
        variant: variant,
        slot: slot,
        starting: starting
    }}
    #[inline(always)]
    pub fn at(&self) -> Option<Symbol> {
        let productions = GRAMMAR.get(&self.rule).unwrap();
        if self.slot as usize == productions[self.variant as usize].len() {None} else {Some(productions[self.variant as usize][self.slot as usize])}
    }
    #[inline(always)]
    pub fn next(&self) -> State {return State {
        rule: self.rule,
        variant: self.variant,
        slot: self.slot + 1,
        starting: self.starting
    }}
}

//> RESOURCES -> FOLLOW
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Backpointer {
    symbol: Symbol,
    start: u32,
    end: u32
} impl Backpointer {
    #[inline(always)]
    pub fn new(symbol: Symbol, start: u32, end: u32) -> Self {return Backpointer {
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
    chart: Vec<FastMap<State, FastSet<SmallVec<[Backpointer; SAVEDFOLLOWERS]>>>>,
    waiting: Vec<FastMap<Symbol, FastSet<State>>>
} impl Parser {
    pub fn new() -> Self {return Parser {
        chart: Vec::new(),
        waiting: Vec::new()
    }}
    #[inline(always)]
    fn wait(&mut self, index: u32, state: &State) -> () {if let Some(symbol) = state.at() {self.waiting[index as usize].entry(symbol).or_default().insert(state.clone());}}
    #[inline(always)]
    fn recall(&mut self, index: u32, state: &State) -> &mut FastSet<SmallVec<[Backpointer; SAVEDFOLLOWERS]>> {return self.chart[index as usize].entry(state.clone()).or_default()}
    #[inline(always)]
    fn review(&mut self, index: u32, state: &State) -> &FastSet<SmallVec<[Backpointer; SAVEDFOLLOWERS]>> {return self.chart[index as usize].entry(state.clone()).or_default()}
    fn best(&self, pool: &FastMap<Backpointer, FastSet<SmallVec<[Backpointer; SAVEDFOLLOWERS]>>>, node: &Backpointer, memory: &mut FastMap<Backpointer, (usize, Option<Partition>)>) -> (usize, Option<Partition>) {
        if let Some(result) = memory.get(node) {return result.clone()};
        if let Symbol::Kind(kind) = node.symbol {
            let result = (0, None);
            memory.insert(*node, result.clone());
            return result;
        };
        let mut bcore = 0;
        let mut btree = None;
        if let Some(derivations) = pool.get(node) {
            for derivation in derivations {
                let mut score = match node.symbol {
                    Symbol::NonTerminal(object) => object.score(),
                    Symbol::Internal(code) => 0,
                    Symbol::Kind(kind) => unreachable!()
                };
                let mut children = Vec::new();
                for child in derivation {
                    let (ccore, ctree) = self.best(pool, child, memory);
                    score += ccore;
                    if let Some(tree) = ctree {
                        match tree {
                            Partition::Internal(items) => children.extend(items),
                            Partition::NonTerminal(items) => children.push(items),
                        }
                    };
                }
                if score > bcore || btree.is_none() {
                    bcore = score;
                    btree = Some(match node.symbol {
                        Symbol::NonTerminal(object) => Partition::NonTerminal(object.summon(children)),
                        Symbol::Internal(code) => Partition::Internal(children),
                        Symbol::Kind(kind) => unreachable!()
                    });
                }
            }
        };
        let result = (bcore, btree);
        memory.insert(*node, result.clone());
        return result;
    }
    fn reset(&mut self, tokens: &mut Vec<BindedToken>) -> () {
        tokens.retain(|token| ORDER.get(&token.kind).unwrap().1 != Responsibility::Null);
        self.chart.clear();
        self.waiting.clear();
        self.chart.extend((0..(tokens.len() + 1)).map(|iteration| FastMap::new()));
        self.waiting.extend((0..(tokens.len() + 1)).map(|iteration| FastMap::new()));
        let root = State::new(Rule::Internal(0), 0, 0, 0);
        self.recall(0, &root).insert(SmallVec::new());
        self.wait(0, &root);
    }
    pub fn run(&mut self, mut tokens: Vec<BindedToken>, settings: &Settings) -> Result<Start, Issue> {
        self.reset(&mut tokens);
        let pool = self.parse(&tokens);
        let Partition::NonTerminal(NonTerminal::Start(root)) = self.best(&pool, &Backpointer::new(Symbol::NonTerminal(Object::Start), 0, tokens.len() as u32), &mut FastMap::new()).1.unwrap() else {unreachable!()};
        return Ok(root);
    }
    fn parse(&mut self, tokens: &Vec<BindedToken>) -> FastMap<Backpointer, FastSet<SmallVec<[Backpointer; SAVEDFOLLOWERS]>>> {
        let length = tokens.len();
        let mut pool = FastMap::new() as FastMap<Backpointer, FastSet<SmallVec<[Backpointer; SAVEDFOLLOWERS]>>>;
        let mut agenda = Deque::new();
        let mut completed = FastSet::new();
        for index in (0 as u32)..((length + 1) as u32) {
            let token = tokens.get(index as usize);
            agenda.extend(self.chart[index as usize].keys().cloned());
            while !agenda.is_empty() {
                let state = agenda.pop_front().unwrap();
                let at = state.at();
                if at.is_none() {
                    for awaiting in self.waiting[state.starting as usize].get(&state.rule.into()).cloned().unwrap_or_default() {
                        let advanced = awaiting.next();
                        let stored = self.review(index, &advanced).len();
                        let pointer = Backpointer::new(state.rule.into(), state.starting, index);
                        pool.entry(pointer).or_default().extend(self.review(index, &state).clone());
                        let backpointers = self.chart[state.starting as usize].get(&awaiting).cloned().unwrap_or_default().into_iter().map(|mut backpointer| {backpointer.push(pointer); backpointer}).collect::<Vec<SmallVec<[Backpointer; SAVEDFOLLOWERS]>>>();
                        self.wait(index, &advanced);
                        let additional = self.recall(index, &advanced);
                        additional.extend(backpointers);
                        if additional.len() > stored {
                            agenda.push_back(advanced);
                            agenda.extend(completed.clone());
                        }
                    }
                    completed.insert(state);
                } else if let (Some(Symbol::Kind(kind)), Some(value)) = (at, token) && kind == value.kind {
                    let addable = self.recall(index, &state).clone().into_iter().map(|mut element| {element.push(Backpointer::new(Symbol::Kind(kind), index, index + 1)); element}).collect::<Vec<SmallVec<[Backpointer; SAVEDFOLLOWERS]>>>();
                    let following = state.next();
                    self.recall(index + 1, &following).extend(addable);
                    self.wait(index + 1, &following);
                } else {
                    let rule = match at.unwrap() {
                        Symbol::Internal(code) => Rule::Internal(code),
                        Symbol::NonTerminal(object) => Rule::NonTerminal(object), 
                        Symbol::Kind(kind) => continue
                    };
                    for variant in (0 as u8)..(GRAMMAR.get(&rule).unwrap().len() as u8) {
                        let possibility = State::new(rule, variant, 0, index);
                        self.wait(index, &possibility);
                        let produced = self.recall(index, &possibility);
                        if produced.is_empty() {
                            produced.insert(SmallVec::new());
                            agenda.push_back(possibility);
                        }
                    }
                }
            }
            completed.clear();
        };
        return pool;
    }
}