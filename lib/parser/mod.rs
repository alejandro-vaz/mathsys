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
    input: &'input [u8]
) -> Start<'input> {return match start(&mut State::from(input)) {
    Ok(start) => start,
    Err(failure) => Implementation::critical(failure)
}}