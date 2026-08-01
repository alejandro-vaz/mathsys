//^
//^ HEAD
//^

//> HEAD -> STD
use std::{
    sync::LazyLock,
    collections::{
        HashMap as Map,
        HashSet as Set
    }
};

//> HEAD -> SUPER
use super::{
    production::Production,
    symbol::Symbol,
    closure::closure,
    goto::goto,
    constants::STATES
};

//> HEAD -> LIBUTILS
use libutils::stack_array::Array;


//^
//^ AUTOMATON
//^

//> AUTOMATON -> STRUCT
pub struct Automaton {
    pub states: [Set<Production>; STATES],
    pub transitions: Map<(usize, &'static Symbol), usize>
}

//> AUTOMATON -> STATIC
pub static AUTOMATON: LazyLock<Automaton> = LazyLock::new(|| {
    let mut states = Array::<_, STATES>::new();
    let mut transitions = Map::new();
    let mut initial = Set::from([Production::default()]);
    closure(&mut initial);
    states.push(initial);
    let mut index = 0;
    while index < states.len() {
        for symbol in states[index].iter().filter_map(|item| {
            item.derivation.get(item.at)
        }).collect::<Set<_>>() {
            let next = goto(&states[index], symbol);
            if !next.is_empty() {transitions.insert(
                (index, symbol), 
                if let Some(position) = states.iter().position(|state| {
                    state == &next
                }) {position} else {
                    states.push(next);
                    states.len() - 1
                }
            );}
        }
        index += 1;
    }
    return Automaton {
        states: states.try_into().unwrap(),
        transitions: transitions
    };
});