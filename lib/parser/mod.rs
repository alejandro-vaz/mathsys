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
        machine.pass(token);
        machine.advance();
    }
    machine.pass(&Token::EndOfFile);
    return machine.finish();
}