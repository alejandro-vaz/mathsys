//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod data;
pub mod parsed;
pub mod pointer;
pub mod state;

//> HEAD -> CRATE
use crate::{
    grammar::{
        constants::DERIVATIONS,
        GRAMMAR,
        rule::Rule,
        symbol::Symbol
    },
    tokenizer::token::Token
};

//> HEAD -> LIBUTILS
use libutils::stack_array::Array;

//> HEAD -> STD
use std::collections::{
    HashMap as Map,
    HashSet as Set,
    VecDeque as Deque
};

//> HEAD -> TYPED_ARENA
use typed_arena::Arena;

//> HEAD -> AMBIGUITY
use data::AMBIGUITY;

//> HEAD -> STATE
use state::State;

//> HEAD -> POINTER
use pointer::Pointer;

//> HEAD -> PARSED
use parsed::Parsed;


//^
//^ PARSER
//^

//> PARSER -> FUNCTION
pub fn parse<'valid>(
    tokens: Vec<Token<'valid>>, 
) -> Map<Pointer<'valid>, Array<Array<Pointer<'valid>, DERIVATIONS>, AMBIGUITY>> {
    let states = Arena::default();
    let pointers = Arena::default();
    let mut chart = Vec::<Map<
        &State, 
        Set<Array<&Pointer<'valid>, DERIVATIONS>>
    >>::with_capacity(tokens.len() + 1);
    let mut waiting = Vec::<Map<&Symbol, Set<&State>>>::with_capacity(tokens.len() + 1);
    chart.resize_with(tokens.len() + 1, Map::new);
    waiting.resize_with(tokens.len() + 1, Map::new);
    let root = states.alloc(State {
        rule: Rule::usize(0),
        variant: &GRAMMAR.get(&Rule::usize(0)).unwrap()[0],
        slot: 0,
        starting: 0
    });
    chart[0].entry(root).or_default().insert(Array::new());
    waiting[0].entry(&root.variant[root.slot]).or_default().insert(root);
    let mut pool = Map::new();
    let mut agenda = Deque::new();
    for index in 0..(tokens.len() + 1) {
        let token = tokens.get(index);
        let mut queue = Set::new();
        let mut seen = Set::new();
        let mut completed = Map::new();
        chart[index].keys().copied().for_each(|state| {
            if !seen.contains(&state) && queue.insert(state) {agenda.push_back(state)}
        });
        while let Some(state) = agenda.pop_front() {
            queue.remove(&state);
            if !seen.insert(state) {continue}
            if state.variant.get(state.slot).is_none() {
                complete(index, state, &mut agenda, &mut queue, &seen, &mut completed, &mut pool, &mut waiting, &mut chart, &states, &pointers);
            } else if let (Some(Symbol::str(kind)), Some(value)) = (state.variant.get(state.slot), token) && *kind == value.as_ref() {
                scan(index, state, value, &mut waiting, &mut chart, &states, &pointers);
            } else {
                predict(index, state, &mut agenda, &mut queue, &seen, &completed, &mut pool, &mut waiting, &mut chart, &states, &pointers);
            }
        }
    }
    return pool.into_iter().map(|(backpointer, set)| (
        backpointer.clone(), 
        set.into_iter().flat_map(|(index, state)| {
            chart[index].get(state).into_iter().flat_map(Set::iter).map(|small| {
                small.iter().map(|pointer| {
                    (*pointer).clone()
                }).collect::<Array<Pointer<'valid>, DERIVATIONS>>()
            })
        }).collect()
    )).collect();
}

//> PARSER -> COMPLETE
fn complete<'valid, 'arena, 'land>(
    index: usize, 
    state: &'arena State, 
    agenda: &mut Deque<&'arena State>, 
    queue: &mut Set<&'arena State>,
    seen: &Set<&'arena State>,
    completed: &mut Map<Symbol, Set<&'arena State>>,
    pool: &mut Map<&'land Pointer<'valid>, Set<(usize, &'arena State)>>,
    waiting: &mut Vec<Map<&'static Symbol, Set<&'arena State>>>, 
    chart: &mut Vec<Map<&'arena State, Set<Array<&'land Pointer<'valid>, DERIVATIONS>>>>,
    states: &'arena Arena<State>,
    pointers: &'land Arena<Pointer<'valid>>
) -> () {
    let symbol = state.rule.into();
    if let Some(waitlist) = waiting[state.starting].get(&symbol).cloned() {for awaiting in waitlist {advance(index, awaiting, state, agenda, queue, seen, completed, pool, waiting, chart, states, pointers)}}
    if state.starting == index {completed.entry(symbol).or_default().insert(state);}
}

//> PARSER -> SCAN
fn scan<'valid, 'arena, 'land>(
    index: usize,
    state: &'arena State,
    token: &Token<'valid>,
    waiting: &mut Vec<Map<&'static Symbol, Set<&'arena State>>>,
    chart: &mut Vec<Map<&'arena State, Set<Array<&'land Pointer<'valid>, DERIVATIONS>>>>,
    states: &'arena Arena<State>,
    pointers: &'land Arena<Pointer<'valid>>
) -> () {
    let addable = chart[index].entry(state).or_default().clone().into_iter().map(|mut element| {element.push(pointers.alloc(Pointer::<'valid> {
        parsed: Parsed::Token(token.clone()),
        start: index,
        end: index + 1
    })); element}).collect::<Vec<Array<&Pointer, DERIVATIONS>>>();
    let following = states.alloc(State {
        rule: state.rule,
        variant: state.variant,
        slot: state.slot + 1,
        starting: state.starting
    });
    chart[index + 1].entry(following).or_default().extend(addable);
    if let Some(symbol) = following.variant.get(following.slot) {waiting[index + 1].entry(symbol).or_default().insert(following);}
}

//> PARSER -> PREDICT
fn predict<'valid, 'arena, 'land>(
    index: usize, 
    state: &'arena State, 
    agenda: &mut Deque<&'arena State>, 
    queue: &mut Set<&'arena State>,
    seen: &Set<&'arena State>,
    completed: &Map<Symbol, Set<&'arena State>>,
    pool: &mut Map<&'land Pointer<'valid>, Set<(usize, &'arena State)>>,
    waiting: &mut Vec<Map<&'static Symbol, Set<&'arena State>>>, 
    chart: &mut Vec<Map<&'arena State, Set<Array<&'land Pointer<'valid>, DERIVATIONS>>>>,
    states: &'arena Arena<State>,
    pointers: &'land Arena<Pointer<'valid>>,
) -> () {
    let rule = match state.variant[state.slot] {
        Symbol::Rule(rule) => rule,
        Symbol::str(_) => return
    };
    for variant in GRAMMAR.get(&rule).unwrap() {
        let possibility = states.alloc(State {
            rule: rule,
            variant: variant,
            slot: 0,
            starting: index
        });
        let produced = chart[index].entry(possibility).or_default();
        if produced.is_empty() {
            produced.insert(Array::new());
            if !seen.contains(possibility) && queue.insert(possibility) {agenda.push_back(possibility)}
        }
        if let Some(symbol) = possibility.variant.get(possibility.slot) {
            if waiting[index].entry(symbol).or_default().insert(possibility) {
                replay(index, possibility, symbol, agenda, queue, seen, completed, pool, waiting, chart, states, pointers);
            }
        }
    }
}

//> PARSER -> REPLAY
fn replay<'valid, 'arena, 'land>(
    index: usize,
    awaiting: &'arena State,
    symbol: &'static Symbol,
    agenda: &mut Deque<&'arena State>,
    enqueued: &mut Set<&'arena State>,
    seen: &Set<&'arena State>,
    completed_here: &Map<Symbol, Set<&'arena State>>,
    pool: &mut Map<&'land Pointer<'valid>, Set<(usize, &'arena State)>>,
    waiting: &mut Vec<Map<&'static Symbol, Set<&'arena State>>>,
    chart: &mut Vec<Map<&'arena State, Set<Array<&'land Pointer<'valid>, DERIVATIONS>>>>,
    arena: &'arena Arena<State>,
    land: &'land Arena<Pointer<'valid>>
) -> () {
    if let Some(completed_states) = completed_here.get(symbol).cloned() {
        for completed in completed_states {
            advance(index, awaiting, completed, agenda, enqueued, seen, completed_here, pool, waiting, chart, arena, land);
        }
    }
}

//> PARSER -> ADVANCE
fn advance<'valid, 'arena, 'land>(
    index: usize,
    awaiting: &'arena State,
    completeds: &'arena State,
    agenda: &mut Deque<&'arena State>,
    enqueued: &mut Set<&'arena State>,
    seen: &Set<&'arena State>,
    completed: &Map<Symbol, Set<&'arena State>>,
    pool: &mut Map<&'land Pointer<'valid>, Set<(usize, &'arena State)>>,
    waiting: &mut Vec<Map<&'static Symbol, Set<&'arena State>>>,
    chart: &mut Vec<Map<&'arena State, Set<Array<&'land Pointer<'valid>, DERIVATIONS>>>>,
    arena: &'arena Arena<State>,
    land: &'land Arena<Pointer<'valid>>
) -> () {
    let advanced = arena.alloc(State {
        rule: awaiting.rule,
        variant: awaiting.variant,
        slot: awaiting.slot + 1,
        starting: awaiting.starting
    });
    let pointer = land.alloc(Pointer {
        parsed: completeds.rule.into(),
        start: completeds.starting,
        end: index
    });
    pool.entry(pointer).or_default().insert((index, completeds));
    let backpointers = if let Some(previous) = chart[completeds.starting].get(awaiting) {previous.iter().map(|back| {
        let mut new = back.clone();
        new.push(pointer);
        new
    }).collect()} else {Vec::new()};
    let additional = chart[index].entry(advanced).or_default();
    if additional.is_empty() {
        additional.extend(backpointers);
        if !seen.contains(advanced) && enqueued.insert(advanced) {agenda.push_back(advanced)}
    } else if let Rule::Object(_) = advanced.rule {
        additional.extend(backpointers);
    }
    if let Some(symbol) = advanced.variant.get(advanced.slot) {
        if waiting[index].entry(symbol).or_default().insert(advanced) {
            replay(index, advanced, symbol, agenda, enqueued, seen, completed, pool, waiting, chart, arena, land);
        }
    }
}