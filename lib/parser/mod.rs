//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod expression;
pub mod factor;
pub mod start;
pub mod state;
pub mod statement;
pub mod term;
pub mod value;

//> HEAD -> CRATE
use crate::{
    syntax::Start, 
    tokenizer::token::Token,
    runtime::Runtime
};

//> HEAD -> STATE
use state::State;

//> HEAD -> START
use start::start;


//^
//^ PARSER
//^

//> PARSER -> FUNCTION6
pub fn parse<'input, Implementation: Runtime<'input>>(
    tokens: Vec<Token<'input>>
) -> Start<'input> {return match start(&mut State::from(tokens)) {
    Ok(start) => start,
    Err(failure) => Implementation::critical(failure)
}}