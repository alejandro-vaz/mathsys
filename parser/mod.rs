//^
//^ HEAD
//^

//> HEAD -> MODULES
pub(super) mod grammar;

//> HEAD -> PRELUDE
use crate::prelude::{
    FastMap, 
    FastSet, 
    Deque, 
    SmallVec
};

//> HEAD -> LOCAL
use self::{
    grammar::{
        GRAMMAR, 
        Rule, 
        Symbol
    },
    super::{
        Settings,
        solver::nonterminal::Object,
        tokenizer::token::{
            BindedToken, 
            ORDER, 
            Responsibility
        }
    }
};


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
        rule: self.rule.clone(),
        variant: self.variant,
        slot: self.slot + 1,
        starting: self.starting
    }}
}

//> RESOURCES -> BACKPOINTER
#[derive(Clone, PartialEq, Eq, Hash)]
pub(super) struct Backpointer<'parsing> {
    pub(super) symbol: Part<'parsing>,
    start: u32,
    end: u32
}

//> RESOURCES -> PART
#[derive(Clone, PartialEq, Eq, Hash)]
pub(super) enum Part<'parsing> {
    NonTerminal(Object),
    Internal(u8),
    Token(BindedToken<'parsing>)
}

//^
//^ PARSER
//^

//> PARSER -> MINPOINTERS
pub(super) static MINPOINTERS: usize = 2;

//> PARSER -> STRUCT
pub(super) struct Parser {} impl Parser {
    pub(super) const fn new() -> Self {return Parser {}}
    #[inline(always)]
    fn wait(&self, index: u32, state: &State, waiting: &mut Vec<FastMap<Symbol, FastSet<State>>>) -> () {if let Some(symbol) = state.at() {waiting[index as usize].entry(symbol).or_default().insert(state.clone());}}
    #[inline(always)]
    fn recall<'exists, 'parsing>(&self, index: u32, state: &State, chart: &'exists mut Vec<FastMap<State, FastSet<SmallVec<[Backpointer<'parsing>; MINPOINTERS]>>>>) -> &'exists mut FastSet<SmallVec<[Backpointer<'parsing>; MINPOINTERS]>> {return chart[index as usize].entry(state.clone()).or_default()}
    #[inline(always)]
    fn review<'exists, 'parsing>(&self, index: u32, state: &State, chart: &'exists mut Vec<FastMap<State, FastSet<SmallVec<[Backpointer<'parsing>; MINPOINTERS]>>>>) -> &'exists FastSet<SmallVec<[Backpointer<'parsing>; MINPOINTERS]>> {return chart[index as usize].entry(state.clone()).or_default()}
    pub(super) fn run<'parsing>(&self, tokens: &Vec<BindedToken<'parsing>>, settings: &Settings) -> FastMap<Backpointer<'parsing>, FastSet<SmallVec<[Backpointer<'parsing>; MINPOINTERS]>>> {
        let mut filtered = tokens.clone();
        filtered.retain(|token| if let Responsibility::Null = ORDER.get(&token.kind).unwrap().1 {false} else {true});
        let mut chart = Vec::new();
        let mut waiting = Vec::new();
        chart.extend((0..(filtered.len() + 1)).map(|iteration| FastMap::new()));
        waiting.extend((0..(filtered.len() + 1)).map(|iteration| FastMap::new()));
        let root = State {
            rule: Rule::Internal(0),
            variant: 0,
            slot: 0,
            starting: 0
        };
        self.recall(0, &root, &mut chart).insert(SmallVec::new());
        self.wait(0, &root, &mut waiting);
        let mut pool = FastMap::new();
        let mut agenda = Deque::new();
        let mut completed = FastSet::new();
        for index in 0..(filtered.len() + 1) {
            let token = filtered.get(index as usize);
            agenda.extend(chart[index as usize].keys().cloned());
            while !agenda.is_empty() {
                let state = agenda.pop_front().unwrap();
                if state.at().is_none() {
                    self.complete(index as u32, state, &mut agenda, &mut completed, &mut pool, &mut waiting, &mut chart);
                } else if let (Some(Symbol::Kind(kind)), Some(value)) = (state.at(), token.cloned()) && kind == value.kind {
                    self.scan(index as u32, state, &mut agenda, value, &mut waiting, &mut chart);
                } else {
                    self.predict(index as u32, state, &mut agenda, &mut waiting, &mut chart);
                }
            }
            completed.clear();
        };
        return pool;
    }
    #[inline(always)]
    fn complete<'parsing>(&self, index: u32, state: State, agenda: &mut Deque<State>, completed: &mut FastSet<State>, pool: &mut FastMap<Backpointer<'parsing>, FastSet<SmallVec<[Backpointer<'parsing>; MINPOINTERS]>>>, waiting: &mut Vec<FastMap<Symbol, FastSet<State>>>, chart: &mut Vec<FastMap<State, FastSet<SmallVec<[Backpointer<'parsing>; MINPOINTERS]>>>>) -> () {
        for awaiting in waiting[state.starting as usize].get(&state.rule.clone().into()).cloned().unwrap_or_default() {
            let advanced = awaiting.next();
            let stored = self.review(index, &advanced, chart).len();
            let pointer = Backpointer {
                symbol: match &state.rule {
                    Rule::Internal(code) => Part::Internal(*code),
                    Rule::NonTerminal(object) => Part::NonTerminal(object.clone())
                },
                start: state.starting,
                end: index
            };
            pool.entry(pointer.clone()).or_default().extend(self.review(index, &state, chart).clone());
            let backpointers = chart[state.starting as usize].get(&awaiting).cloned().unwrap_or_default().into_iter().map(|mut backpointer| {backpointer.push(pointer.clone()); backpointer}).collect::<Vec<SmallVec<[Backpointer; MINPOINTERS]>>>();
            self.wait(index, &advanced, waiting);
            let additional = self.recall(index, &advanced, chart);
            additional.extend(backpointers);
            if additional.len() > stored {
                agenda.push_back(advanced);
                agenda.extend(completed.clone());
            }
        }
        completed.insert(state);
    }
    #[inline(always)]
    fn scan<'parsing>(&self, index: u32, state: State, agenda: &mut Deque<State>, token: BindedToken<'parsing>, waiting: &mut Vec<FastMap<Symbol, FastSet<State>>>, chart: &mut Vec<FastMap<State, FastSet<SmallVec<[Backpointer<'parsing>; MINPOINTERS]>>>>) -> () {
        let addable = self.recall(index, &state, chart).clone().into_iter().map(|mut element| {element.push(Backpointer {
            symbol: Part::Token(token.clone()),
            start: index, 
            end: index + 1
        }); element}).collect::<Vec<SmallVec<[Backpointer; MINPOINTERS]>>>();
        let following = state.next();
        self.recall(index + 1, &following, chart).extend(addable);
        self.wait(index + 1, &following, waiting);
    }
    #[inline(always)]
    fn predict(&self, index: u32, state: State, agenda: &mut Deque<State>, waiting: &mut Vec<FastMap<Symbol, FastSet<State>>>, chart: &mut Vec<FastMap<State, FastSet<SmallVec<[Backpointer; MINPOINTERS]>>>>) -> () {
        let rule = match state.at().unwrap() {
            Symbol::Internal(code) => Rule::Internal(code),
            Symbol::NonTerminal(object) => Rule::NonTerminal(object), 
            Symbol::Kind(kind) => return
        };
        for variant in 0..GRAMMAR[&rule].len() {
            let possibility = State {
                rule: rule.clone(),
                variant: variant as u8,
                slot: 0,
                starting: index
            };
            self.wait(index, &possibility, waiting);
            let produced = self.recall(index, &possibility, chart);
            if produced.is_empty() {
                produced.insert(SmallVec::new());
                agenda.push_back(possibility);
            }
        }
    }
}