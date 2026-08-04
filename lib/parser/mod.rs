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
pub mod machine;
pub mod object;
pub mod production;
pub mod rule;
pub mod symbol;
pub mod tables;

//> HEAD -> CRATE
use crate::tokenizer::token::Token;

//> HEAD -> MACHINE
use machine::Machine;


//^
//^ PARSER
//^

//> PARSER -> FUNCTION6
pub fn parse<'input>(tokens: &'input Vec<Token<'input>>) -> () {
    let mut machine = Machine::default();
    for token in tokens {
        machine.pass(token);
        machine.advance();
    }
    machine.pass(&Token::EndOfFile);
    return machine.finish();
}