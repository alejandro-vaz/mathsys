//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    HashMap, HashSet, VecDeque
};

//> HEAD -> LOCAL
use super::nonterminal::NonTerminals;
use super::grammar::{Rule, Symbol, Nexus, SYNTAX};
use super::tokenizer::{Token, ORDER, Responsibility};


//^
//^ RESOURCES
//^

//> RESOURCES -> STATE
#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    rule: Rule,
    productions: Vec<Symbol>,
    slot: u8,
    starting: u16
} impl State {
    #[inline(always)]
    fn at(&self) -> Option<Symbol> {return self.productions.get(self.slot as usize).cloned()}
    #[inline(always)]
    fn full(&self) -> bool {return (self.slot as usize) == self.productions.len()}
    #[inline(always)]
    fn new(rule: Rule, productions: Vec<Symbol>, slot: u8, starting: u16) -> State {return State {
        rule: rule,
        productions: productions,
        slot: slot,
        starting: starting
    }}
}

//> RESOURCES -> SPPF
#[derive(PartialEq, Eq, Hash, Clone)]
struct SPPF {
    nexus: Nexus,
    start: u16,
    end: u16
} impl SPPF {pub fn new(nexus: Nexus, start: u16, end: u16) -> Self {return SPPF {
    nexus: nexus,
    start: start,
    end: end
}}}


//^
//^ PARSER
//^

//> PARSER -> STRUCT
pub struct Parser {
    chart: Vec<HashMap<State, HashSet<Vec<SPPF>>>>,
    pool: HashMap<SPPF, HashSet<Vec<SPPF>>>,
    waiting: Vec<HashMap<Symbol, HashSet<State>>>
} impl Parser {
    pub fn new() -> Parser {return Parser {
        chart: Vec::new(),
        pool: HashMap::new(),
        waiting: Vec::new()
    }}
    fn reset(&mut self, mut tokens: Vec<Token>) -> Vec<Token> {
        tokens = tokens.into_iter().filter(|token| ORDER.get(&token.kind).unwrap().1 != Responsibility::Null).collect();
        let length = tokens.len();
        self.chart.clear();
        self.pool.clear();
        self.waiting.clear();
        for index in 0..(tokens.len() + 1) {
            self.chart.push(HashMap::new());
            self.waiting.push(HashMap::new());
        };
        self.recall(0, State::new(Rule::Internal(0), Vec::from([Symbol::NonTerminals(NonTerminals::Start)]), 0, 0));
        return tokens;
    }
    fn recall(&mut self, index: u16, state: State) -> &mut HashSet<Vec<SPPF>> {
        if let Some(symbol) = state.at() {
            self.waiting[index as usize].entry(symbol).or_insert_with(HashSet::new).insert(state.clone());
        }
        return self.chart[index as usize].entry(state).or_insert_with(HashSet::new);
    }
    pub fn run(&mut self, mut tokens: Vec<Token>) -> bool {
        tokens = self.reset(tokens);
        self.parse(tokens);
        return true;
    }
    fn parse(&mut self, tokens: Vec<Token>) -> () {
        let length = tokens.len();
        for index in (0 as u16)..((length + 1) as u16) {
            let token = if length > (index as usize) {Some(tokens[index as usize].clone())} else {None};
            let mut agenda = VecDeque::from(self.chart[index as usize].keys().cloned().collect::<Vec<State>>());
            //let mut completed = HashSet::new();
            while !agenda.is_empty() {
                let state = agenda.pop_back().unwrap();
                if state.full() {
                    // complete
                } else if let (Some(existing), Some(Symbol::Kind(kind))) = (&token, state.at()) && existing.kind == kind {
                    // scan
                } else {
                    //let mut changed = Vec::new();
                    // predict
                }
            }
        };
    }
}