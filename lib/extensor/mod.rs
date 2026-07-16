//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod types;

//> HEAD -> LIBUTILS
use libutils::stack_array::Array;

//> HEAD -> STD
use std::collections::HashMap as Map;

//> HEAD -> TYPES
use types::{
    Rule,
    Symbol,
    DERIVATIONS,
    LENGTH
};

//> HEAD -> CRATE
use crate::reducer::{
    WIDTH,
    DELIMITER
};


//^
//^ EXTENSOR
//^

//> EXTENSOR -> FUNCTION
pub fn extend(
    grammar: [&'static str; WIDTH]
) -> Map<Rule, Array<Array<Symbol, LENGTH>, DERIVATIONS>> {
    let mut map = Map::new();
    for line in grammar {
        let (rule, productions) = line.split_once(Into::<&'static str>::into(DELIMITER)).unwrap();
        let rule = rule.into();
        for variant in productions.split('|').map(str::trim) {
            map.entry(rule).or_insert_with(Array::new).push(if variant.is_empty() {
                Array::new()
            } else {
                variant.split(' ').map(Symbol::from).collect()
            })
        }
    }
    return map;
}