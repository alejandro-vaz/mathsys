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
        state.advance(|byte| byte == b'^')?;
        let expression = expression(state)?;
        state.advance(|byte| byte == b'^')?;
        Ok(expression)
    })
})}

//> FACTOR -> LIMIT
pub fn limit<'input>(
    state: &mut State<'input>
) -> Result<Limit<'input>, Failure<'input>> {
    state.advance(|byte| byte == b'l')?;
    state.advance(|byte| byte == b'i')?;
    state.advance(|byte| byte == b'm')?;
    let identifier = identifier(state)?;
    state.advance(|byte| byte == b'-')?;
    state.advance(|byte| byte == b'>')?;
    let approach = expression(state)?;
    let direction = state.optional(|state| {
        state.advance(|byte| matches!(byte, b'+' | b'-')).map(|sign| sign == b'+')
    });
    state.advance(|byte| byte == b'o')?;
    state.advance(|byte| byte == b'f')?;
    return Ok(Limit {
        identifier: identifier,
        expression: approach,
        direction: direction,
        nest: nest(state)?,
        exponent: state.optional(|state| {
            state.advance(|byte| byte == b'^')?;
            let expression = expression(state)?;
            state.advance(|byte| byte == b'^')?;
            Ok(expression)
        })
    })
}