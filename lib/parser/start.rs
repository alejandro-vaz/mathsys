//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    state::State,
    statement::statement
};

//> HEAD -> CRATE
use crate::{
    failure::Failure,
    syntax::Start
};


//^
//^ START
//^

//> START -> FUNCTION
pub fn start<'input>(state: &mut State<'input>) -> Result<Start<'input>, Failure<'input>> {
    let statements = state.optional(|state| {
        state.skip(b'\n');
        let first = statement(state)?;
        let mut rest = state.multiple(|state| {
            state.skip(b'\n');
            statement(state)
        });
        rest.insert(0, first);
        Ok(rest)
    }).unwrap_or_default();
    state.skip(b'\n');
    return state.depleted(Start {
        statements: statements
    });
}