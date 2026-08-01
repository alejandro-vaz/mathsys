//^
//^ HEAD
//^

//> HEAD -> STD
use std::collections::HashSet as Set;

//> HEAD -> SUPER
use super::{
    production::Production,
    grammar::GRAMMAR
};


//^
//^ CLOSURE
//^

//> CLOSURE -> FUNCTION
pub fn closure(items: &mut Set<Production>) -> () {
    let mut length = items.len();
    loop {
        for rule in items.iter().filter_map(|item| {
            item.derivation.get(item.at)?.as_rule()
        }).collect::<Set<_>>() {for derivation in &GRAMMAR[&rule] {items.insert(Production {
            rule: rule,
            derivation: derivation,
            at: 0
        });}}
        if items.len() == length {break}
        length = items.len();
    }
}