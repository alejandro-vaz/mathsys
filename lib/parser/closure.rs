//^
//^ HEAD
//^

//> HEAD -> STD
use std::collections::HashSet as Set;

//> HEAD -> SUPER
use super::{
    production::Production,
    grammar::GRAMMAR,
    first::first
};


//^
//^ CLOSURE
//^

//> CLOSURE -> FUNCTION
pub fn closure(items: &mut Set<Production>) -> () {
    let mut length = items.len();
    loop {
        for (rule, lookaheads) in items.iter().filter_map(|item| Some((
            item.derivation.get(item.at)?.as_rule()?, 
            first(&item.derivation[item.at + 1 ..], item.lookahead)
        ))).collect::<Vec<_>>() {for derivation in GRAMMAR[rule].iter() {
            for &lookahead in &lookaheads {
                items.insert(Production {
                    rule: rule,
                    derivation: derivation,
                    at: 0,
                    lookahead: lookahead
                });
            }
        }}
        if items.len() == length {break}
        length = items.len();
    }
}