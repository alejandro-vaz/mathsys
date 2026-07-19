//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod bnf;
pub mod constants;
pub mod ebnf;
pub mod object;
pub mod rule;
pub mod symbol;

//> HEAD -> LIBUTILS
use libutils::stack_array::Array;

//> HEAD -> STD
use std::{
    collections::HashMap as Map,
    sync::LazyLock
};

//> HEAD -> CONSTANTS
use constants::{
    DERIVATIONS,
    DERIVATION_LENGTH
};

//> HEAD -> RULE
use rule::Rule;

//> HEAD -> SYMBOL
use symbol::Symbol;

//> HEAD -> BNF
use bnf::BNF;


//^
//^ GRAMMAR
//^

//> GRAMMAR -> STATIC
pub static GRAMMAR: LazyLock<Map<
    Rule, 
    Array<Array<Symbol, DERIVATION_LENGTH>, DERIVATIONS>
>> = LazyLock::new(|| {
    let mut map = Map::new();
    for line in *BNF.split('\n').collect::<Vec<&'static str>>().into_array::<49>().unwrap() {
        let (rule, productions) = line.split_once(" -> ").unwrap();
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
});