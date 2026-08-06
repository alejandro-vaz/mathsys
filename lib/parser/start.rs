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
        state.optional(|state| state.advance(|token| token.is_newlines().then_some(())));
        let first = statement(state)?;
        let mut rest = state.multiple(|state| {
            state.advance(|token| token.is_newlines().then_some(()))?;
            statement(state)
        });
        rest.insert(0, first);
        Ok(rest)
    }).unwrap_or_default();
    state.optional(|state| state.advance(|token| token.is_newlines().then_some(())));
    return state.advance(|token| token.is_end_of_file().then_some(Start {
        statements: statements
    }));
}