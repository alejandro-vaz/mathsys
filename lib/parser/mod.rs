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
pub mod head;
pub mod object;
pub mod parse;
pub mod production;
pub mod rule;
pub mod stack;
pub mod state;
pub mod symbol;
pub mod tables;
pub mod trace;

//> HEAD -> CRATE
use crate::tokenizer::token::Token;

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

//> HEAD -> CORE
use core::hint::unreachable_unchecked;

//> HEAD -> HEAD
use head::Head;

//> HEAD -> FOREST
use parse::Parse;


//^
//^ PARSER
//^

//> PARSER -> FUNCTION6
pub fn parse<'input>(tokens: Vec<Token<'input>>) -> Parse {
    let mut index = 0;
    let mut stack = Stack::default();
    loop {
        while let Some(head) = stack.next() {for action in ACTION[stack.get(head.state)].get(
            tokens[index].as_ref()
        ).map(Array::as_ref).unwrap_or_default() {match action {
            Action::Reduce {rule, length} => {
                for base in stack.frontier(head, *length) {
                    let rawstate = *match GOTO[stack.get(base.state)].get(rule) {
                        None => continue,
                        Some(rawstate) => rawstate
                    };
                    let nextstate = stack.state(rawstate, index);
                    let next = Head {
                        state: nextstate,
                        trace: stack.trace(action, index, head.trace, base.trace, nextstate)
                    };
                    stack.reduce(base, head.trace, next);
                }
            }
            Action::Shift {goto} => {
                let nextstate = stack.state(*goto, index + 1);
                let next = Head {
                    state: nextstate,
                    trace: stack.trace(action, index, head.trace, head.trace, nextstate)
                };
                stack.shift(head, next);
            }
            Action::Accept => {
                let trace = stack.trace(action, index, head.trace, head.trace, head.state);
                stack.accept(head.trace, trace);
            }
            Action::Start => unsafe {unreachable_unchecked()}
        }}}
        if !stack.advance() {break};
        index += 1;
    }
    return stack.finish();
}
