//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod action;
pub mod automaton;
pub mod bnf;
pub mod closure;
pub mod constants;
pub mod ebnf;
pub mod forest;
pub mod goto;
pub mod grammar;
pub mod machine;
pub mod object;
pub mod parsed;
pub mod production;
pub mod rule;
pub mod symbol;
pub mod tables;

//> HEAD -> CRATE
use crate::tokenizer::token::Token;

//> HEAD -> TABLES
use tables::ACTION;

//> HEAD -> ACTION
use action::Action;

//> HEAD -> LIBUTILS
use libutils::stack_array::Array;

//> HEAD -> MACHINE
use machine::Machine;

//> HEAD -> FOREST
use forest::Forest;


//^
//^ PARSER
//^

//> PARSER -> FUNCTION6
pub fn parse<'input>(tokens: &'input Vec<Token<'input>>) -> Forest<'input> {
    let mut machine = Machine::default();
    for token in tokens {
        let name = token.as_ref();
        while let Some(state) = machine.next() {for action in ACTION[machine.get(state)].get(
            name
        ).map(Array::as_ref).unwrap_or_default() {match action {
            Action::Reduce {rule, length} => machine.reduce(state, length, rule),
            Action::Shift {goto} => machine.shift(state, token, goto),
            Action::Accept => machine.accept(state)
        }}}
        machine.advance();
    }
    return machine.finish();
}
