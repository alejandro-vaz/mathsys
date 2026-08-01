//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    production::Production,
    symbol::Symbol,
    closure::closure
};

//> HEAD -> STD
use std::collections::HashSet as Set;


//^
//^ GOTO
//^

//> GOTO -> FUNCTION
pub fn goto(items: &Set<Production>, symbol: &'static Symbol) -> Set<Production> {
    let mut following = Set::new();
    for item in items {if let Some(next) = item.derivation.get(item.at) && next == symbol {
        following.insert(Production {
            rule: item.rule,
            derivation: item.derivation,
            at: item.at + 1,
            lookahead: item.lookahead
        });
    }};
    closure(&mut following);
    return following;
}