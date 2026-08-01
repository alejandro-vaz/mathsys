//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    symbol::Symbol,
    grammar::GRAMMAR
};

//> HEAD -> STD
use std::collections::HashSet as Set;

//> HEAD -> LIBUTILS
use libutils::stack_array::Array;


//^
//^ FIRST
//^

//> FIRST -> FUNCTION
pub fn first(derivation: &'static [Symbol], lookahead: &'static str) -> Set<&'static str> {
    let mut tokens = Set::new();
    inner(derivation, lookahead, &mut tokens);
    return tokens;
}

//> FIRST -> FUNCTION
fn inner(
    symbols: &'static [Symbol], 
    lookahead: &'static str,
    tokens: &mut Set<&'static str>
) -> bool {
    return match symbols.first() {
        None => {
            tokens.insert(lookahead);
            true
        },
        Some(Symbol::str(token)) => {
            tokens.insert(token);
            false
        },
        Some(Symbol::Rule(rule)) => {
            let mut nullable = false;
            for production in GRAMMAR[rule].iter().map(Array::as_ref) {
                if inner(production, lookahead, tokens) {nullable = true}
            }
            if nullable {inner(&symbols[1..], lookahead, tokens)} else {false}
        }
    }
}