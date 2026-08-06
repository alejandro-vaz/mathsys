//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    state::State,
    value::{
        value,
        identifier,
        nest
    }
};

//> HEAD -> CRATE
use crate::{
    failure::Failure, 
    parser::expression::expression, 
    syntax::factor::{
        Factor, 
        Limit, 
        Raised
    }
};


//^
//^ FACTOR
//^

//> FACTOR -> DISPATCH
pub fn factor<'input>(
    state: &mut State<'input>
) -> Result<Factor<'input>, Failure<'input>> {return match state.optional(raised) {
    None => match state.optional(limit) {
        Some(limit) => Ok(Factor::Limit(limit)),
        None => Err(Failure::CouldntParseFactor)
    },
    Some(raised) => Ok(Factor::Raised(raised))
}}

//> FACTOR -> RAISED
pub fn raised<'input>(
    state: &mut State<'input>
) -> Result<Raised<'input>, Failure<'input>> {return Ok(Raised {
    value: value(state)?,
    exponent: state.optional(|state| {
        state.advance(|token| token.is_exponentiation().then_some(()))?;
        let expression = expression(state)?;
        state.advance(|token| token.is_exponentiation().then_some(expression))
    })
})}

//> FACTOR -> LIMIT
pub fn limit<'input>(
    state: &mut State<'input>
) -> Result<Limit<'input>, Failure<'input>> {
    state.advance(|token| token.is_limit().then_some(()))?;
    let identifier = identifier(state)?;
    state.advance(|token| token.is_to().then_some(()))?;
    let approach = expression(state)?;
    let direction = state.optional(|state| state.advance(|token| {
        token.as_sign().map(|sign| *sign)
    }));
    state.advance(|token| token.is_of().then_some(()))?;
    return Ok(Limit {
        identifier: identifier,
        expression: approach,
        direction: direction,
        nest: nest(state)?,
        exponent: state.optional(|state| {
            state.advance(|token| token.is_exponentiation().then_some(()))?;
            let expression = expression(state)?;
            state.advance(|token| token.is_exponentiation().then_some(expression))
        })
    })
}