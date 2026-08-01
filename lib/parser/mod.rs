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
pub mod first;
pub mod goto;
pub mod grammar;
pub mod head;
pub mod object;
pub mod production;
pub mod rule;
pub mod stack;
pub mod state;
pub mod symbol;
pub mod tables;
pub mod trace;

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

//> HEAD -> CORE
use core::hint::unreachable_unchecked;

//> HEAD -> HEAD
use head::Head;


//^
//^ PARSER
//^

//> PARSER -> FUNCTION6
pub fn parse<'input>(tokens: Vec<Token<'input>>) -> Start<'input> {
    let mut index = 0;
    let mut stack = Stack::default();
    'parsing: loop {
        let token = &tokens[index];
        while let Some(head) = stack.next() {for action in ACTION[stack.get(head.state)].get(
            token.as_ref()
        ).map(Array::as_ref).unwrap_or_default() {match action {
            Action::Reduce {rule, length} => {
                for state in stack.frontier(head.state, *length) {
                    let rawstate = GOTO[stack.get(state)][rule];
                    let next = Head {
                        state: stack.state(rawstate, index),
                        trace: stack.trace(action, index)
                    };
                    stack.reduce(state, head.trace, next);
                }
            }
            Action::Shift {goto} => {
                let next = Head {
                    state: stack.state(*goto, index + 1),
                    trace: stack.trace(action, index)
                };
                stack.shift(head, next);
            }
            Action::Accept => break 'parsing,
            Action::Start => unsafe {unreachable_unchecked()}
        }}}
        if !stack.advance() {break};
        index += 1;
    }
    Start {
        stream: Vec::new()
    }
}