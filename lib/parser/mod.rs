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
pub mod goto;
pub mod grammar;
pub mod object;
pub mod production;
pub mod rule;
pub mod stack;
pub mod symbol;
pub mod tables;

//> HEAD -> CRATE
use crate::{
    tokenizer::token::Token,
    syntax::Start
};

//> HEAD -> TABLES
use tables::{
    ACTION,
    GOTO
};

//> HEAD -> ACTION
use action::Action;

//> HEAD -> LIBUTILS
use libutils::stack_array::Array;

//> HEAD -> STACK
use stack::Stack;


//^
//^ PARSER
//^

//> PARSER -> FUNCTION6
pub fn parse<'input>(tokens: Vec<Token<'input>>) -> Start<'input> {
    let mut index = 0;
    let mut stack = Stack::default();
    loop {
        while let Some(node) = stack.next() {for action in ACTION[stack.get(node)].get(
            tokens[index].as_ref()
        ).map(Array::as_ref).unwrap_or_default() {match action {
            Action::Reduce {rule, length} => {
                for state in stack.frontier(node, *length) {
                    let rawstate = GOTO[stack.get(state)][rule];
                    let next = stack.state(rawstate, index);
                    stack.reduce(state, next);
                }
            }
            Action::Shift {goto} => {
                let next = stack.state(*goto, index + 1);
                stack.shift(node, next);
            }
            Action::Accept => stack.accept()
        }}}
        if !stack.advance() {break};
        index += 1;
    }
    return Start {
        stream: Vec::new()
    };
}
