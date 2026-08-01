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
    syntax::Start,
    failure::Failure
};

//> HEAD -> TABLES
use tables::{
    ACTION,
    GOTO
};

//> HEAD -> ACTION
use action::Action;

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::Report,
    systemio::SystemIO,
    stack_array::Array
};

//> HEAD -> STACK
use stack::Stack;

//> HEAD -> CORE
use core::hint::unreachable_unchecked;

//> HEAD -> HEAD
use head::Head;


//^
//^ PARSER
//^

////
////
////
//// brother, todo
/// 
/// remake GLR good
/// with the gss (deleted file)
/// and see how the old earley parser handled all this pointer bullshit to build sppf
/// 
/// copy and reverse engineer what you had
////
////
////

//> PARSER -> FUNCTION6
///
///
/// bro what the hell just pass the interpreter reference directly, 
/// it will save you a lot of headaches
///
///
pub fn parse<'input>(
    tokens: Vec<Token<'input>>, 
    mut report: Report<"Parser">,
    systemio: &'input SystemIO<Failure<'input>>,
    resolver: &'input fn(&'input str, Report<"Resolver">) -> &'input [u8],
    filename: &'input str
) -> Start<'input> {
    let mut index = 0;
    let mut stack = Stack::default();
    'parsing: loop {
        let token = &tokens[index];
        while let Some(head) = stack.next() {
            stack.stat(index);
            for action in ACTION[stack.get(head.state)].get(
                token.as_ref()
            ).map(Array::as_ref).unwrap_or_default() {match action {
                Action::Reduce {rule, length} => {
                    for state in stack.frontier(head.state, *length) {
                        let rawstate = GOTO[stack.get(state)][rule];
                        let next = Head {
                            state: stack.new_state(rawstate, index),
                            trace: stack.new_action(action, index)
                        };
                        println!(
    "reduce {:?} len={} -> {:?}",
    rule,
    length,
    next.state
);
                        stack.reduce(state, head.trace, next);
                    }
                }
                Action::Shift {goto} => {
                    let next = Head {
                        state: stack.new_state(*goto, index + 1),
                        trace: stack.new_action(action, index)
                    };
                    stack.shift(head, next);
                }
                Action::Accept => {
                    println!("success");
                    break 'parsing
                },
                Action::Start => unsafe {unreachable_unchecked()}
            }}
        }
        if !stack.advance() {break};
        index += 1;
    }
    unreachable!()
}