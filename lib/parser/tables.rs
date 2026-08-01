//^
//^ HEAD
//^

//> HEAD -> STD
use std::{
    sync::LazyLock,
    collections::HashMap as Map
};

//> HEAD -> STRUM
use strum::VariantNames;

//> HEAD -> SUPER
use super::{
    automaton::AUTOMATON,
    constants::{
        STATES,
        CONFLICTS
    },
    rule::Rule,
    action::Action,
    symbol::Symbol,
    object::Object
};

//> HEAD -> CORE
use core::array::from_fn as arrayfn;

//> HEAD -> CRATE
use crate::tokenizer::token::Token;

//> HEAD -> LIBUTILS
use libutils::stack_array::Array;


//^
//^ TABLES
//^

//> TABLES -> GOTO
pub static GOTO: LazyLock<[Map<&'static Rule, usize>; STATES]> = LazyLock::new(|| {
    let mut table = arrayfn(|_| Map::new());
    for (&(state, symbol), &next) in &AUTOMATON.transitions {
        if let Symbol::Rule(rule) = symbol {table[state].insert(rule, next);}
    };
    return table;
});

//> TABLES -> ACTION
pub static ACTION: LazyLock<[Map<&'static str, Array<Action, CONFLICTS>>; STATES]> = LazyLock::new(|| {
    let mut actions = arrayfn::<
        Map<&'static str, Array<Action, CONFLICTS>>, 
        STATES, 
        _
    >(|_| Map::new());
    for (&(state, symbol), &next) in &AUTOMATON.transitions {
        if let Symbol::str(token) = symbol {
            let array = actions[state].entry(token).or_default();
            let action = Action::Shift {
                goto: next
            };
            if !array.contains(&action) {array.push(action)}
        }
    };
    for (index, state) in AUTOMATON.states.iter().enumerate() {for item in state {
        if item.at != item.derivation.len() {continue}
        if let Rule::Object(Object::Start) = item.rule && item.lookahead == Token::EndOfFile.as_ref() {actions[index].entry(Token::EndOfFile.as_ref()).or_default().push(Action::Accept)} else {for &token in Token::VARIANTS {
            let array = actions[index].entry(token).or_default();
            let action = Action::Reduce {
                rule: item.rule, 
                length: item.derivation.len()
            };
            if !array.contains(&action) {array.push(action)}
        }}
    }}
    return actions;
});