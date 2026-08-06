//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    state::State,
    factor::factor
};

//> HEAD -> CRATE
use crate::{
    failure::Failure,
    syntax::term::Term
};


//^
//^ TERM
//^

//> TERM -> FUNCTION
pub fn term<'input>(
    state: &mut State<'input>
) -> Result<Term<'input>, Failure<'input>> {
    let mut numerator = Vec::from([factor(state)?]);
    let mut denominator = Vec::new();
    let mut position = true;
    for (change, factor) in state.multiple(|state| {
        let operator = state.optional(|state| {
            state.advance(|token| token.as_operator().copied())
        });
        Ok((operator, factor(state)?))
    }) {
        if let Some(new) = change {position = new}
        match position {
            false => &mut denominator,
            true => &mut numerator
        }.push(factor);
    }
    return Ok(Term {
        numerator: numerator,
        denominator: denominator
    })
}