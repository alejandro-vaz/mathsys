//^
//^ HEAD
//^

//> HEAD -> STD
use std::{
    sync::LazyLock,
    collections::HashMap as Map
};

//> HEAD -> SUPER
use super::{
    rule::Rule,
    symbol::Symbol,
    bnf::BNF,
    constants::{
        DELIMITER,
        DERIVATIONS,
        DERIVATION_LENGTH
    }
};

//> HEAD -> LIBUTILS
use libutils::stack_array::Array;


//^
//^ GRAMMAR
//^

//> GRAMMAR -> STATIC
pub static GRAMMAR: LazyLock<Map<
    Rule, 
    Array<Array<Symbol, DERIVATION_LENGTH>, DERIVATIONS>
>> = LazyLock::new(|| {
    let mut map = Map::new();
    for line in BNF.split('\n') {
        let (rule, productions) = line.split_once(DELIMITER).unwrap();
        for variant in productions.split('|').map(str::trim) {
            map.entry(rule.try_into().unwrap()).or_insert_with(Array::new).push(if variant.is_empty() {
                Array::new()
            } else {
                variant.split(' ').map(Symbol::from).collect()
            })
        }
    }
    return map;
});