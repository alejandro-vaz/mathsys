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
        state.advance(|token| token.as_sign().copied())
    });
    let first = term(state)?;
    let mut terms = state.multiple(|state| {
        let signs = state.more(|state| state.advance(|token| token.as_sign().copied()))?;
        Ok((signs, term(state)?))
    });
    terms.insert(0, (signs, first));
    return Ok(Expression {
        terms: terms
    });
}