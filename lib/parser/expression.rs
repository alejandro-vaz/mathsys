//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    state::State,
    term::term
};

//> HEAD -> CRATE
use crate::{
    syntax::expression::Expression,
    failure::Failure
};


//^
//^ EXPRESSION
//^

//> EXPRESSION -> FUNCTION
pub fn expression<'input>(
    state: &mut State<'input>
) -> Result<Expression<'input>, Failure<'input>> {
    let signs = state.multiple(|state| {
        state.advance(|byte| matches!(byte, b'+' | b'-')).map(|sign| sign == b'+')
    });
    let first = term(state)?;
    let mut terms = state.multiple(|state| Ok((state.more(|state| {
        state.advance(|byte| matches!(byte, b'+' | b'-')).map(|sign| sign == b'+')
    })?, term(state)?)));
    terms.insert(0, (signs, first));
    return Ok(Expression {
        terms: terms
    });
}