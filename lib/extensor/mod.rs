//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod grammar;
pub mod types;

//> HEAD -> LIBUTILS
use libutils::{
    report::{
        Report,
        Name
    },
    array::Array
};

//> HEAD -> STD
use std::collections::HashMap as Map;

//> HEAD -> TYPES
use types::{
    Rule,
    Symbol,
    DERIVATIONS,
    LENGTH
};

//> HEAD -> GRAMMAR
use grammar::GRAMMAR;


//^
//^ EXTENSOR
//^

//> EXTENSOR -> FUNCTION
pub fn extend(
    _report: Report<Name<"Extensor">>
) -> Option<Map<Rule, Array<Array<Symbol, LENGTH>, DERIVATIONS>>> {
    let mut map = Map::new();
    for line in GRAMMAR {
        let (rule, productions) = line.split_once(':').unwrap();
        let rule = rule.into();
        for variant in productions.split('|').map(str::trim) {
            map.entry(rule).or_insert_with(Array::new).push(if variant.is_empty() {
                Array::new()
            } else {
                variant.split(' ').map(Symbol::from).collect()
            })
        }
    }
    return Some(map);
}