//^ 
//^ HEAD
//^ 

//> HEAD -> SUPER
use super::{
    state::State,
    expression::expression
};

//> HEAD -> SYNTAX
use crate::{
    syntax::value::{
        Value,
        Infinite,
        Identifier,
        Nest,
        Vector,
        Number,
        Absolute,
        Undefined,
        Call
    },
    failure::Failure
};


//^
//^ VALUE
//^

//> VALUE -> DISPATCH
pub fn value<'input>(
    state: &mut State<'input>
) -> Result<Value<'input>, Failure<'input>> {return match state.optional(infinite) {
    None => match state.optional(identifier) {
        None => match state.optional(nest) {
            None => match state.optional(vector) {
                None => match state.optional(number) {
                    None => match state.optional(absolute) {
                        None => match state.optional(undefined) {
                            None => match state.optional(call) {
                                Some(call) => Ok(Value::Call(call)),
                                None => Err(Failure::CouldntParseValue)
                            },
                            Some(undefined) => Ok(Value::Undefined(undefined))
                        },
                        Some(absolute) => Ok(Value::Absolute(absolute))
                    },
                    Some(number) => Ok(Value::Number(number))
                },
                Some(vector) => Ok(Value::Vector(vector))
            },
            Some(nest) => Ok(Value::Nest(nest))
        },
        Some(identifier) => Ok(Value::Identifier(identifier))
    },
    Some(infinite) => Ok(Value::Infinite(infinite))
}}

//> VALUE -> INFINITE
pub fn infinite<'input>(state: &mut State<'input>) -> Result<Infinite, Failure<'input>> {
    state.advance(|byte| byte == b'i')?;
    state.advance(|byte| byte == b'n')?;
    state.advance(|byte| byte == b'f')?;
    return Ok(Infinite);
}

//> VALUE -> IDENTIFIER
pub fn identifier<'input>(
    state: &mut State<'input>
) -> Result<Identifier<'input>, Failure<'input>> {return state.record(|byte| {
    matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'$'..=b'%')
}).map(|name| Identifier {
    name: name
})}

//> VALUE -> NEST
pub fn nest<'input>(state: &mut State<'input>) -> Result<Nest<'input>, Failure<'input>> {
    state.advance(|byte| byte == b'(')?;
    let inside = state.optional(expression);
    state.advance(|byte| byte == b')')?;
    return Ok(Nest {
        inside: inside
    });
}

//> VALUE -> VECTOR
pub fn vector<'input>(
    state: &mut State<'input>
) -> Result<Vector<'input>, Failure<'input>> {
    state.advance(|byte| byte == b'[')?;
    let expressions = state.optional(|state| {
        let first = expression(state)?;
        let mut rest = state.multiple(|state| {
            state.advance(|byte| byte == b',')?;
            expression(state)
        });
        rest.insert(0, first);
        Ok(rest)
    }).unwrap_or_default();
    state.advance(|byte| byte == b']')?;
    return Ok(Vector {
        expressions: expressions
    });
}

//> VALUE -> NUMBER
pub fn number<'input>(
    state: &mut State<'input>
) -> Result<Number<'input>, Failure<'input>> {
    return state.record(|byte| matches!(byte, b'0'..=b'9' | b'_')).map(|number| Number {
        number: number
    });
}

//> VALUE -> ABSOLUTE
pub fn absolute<'input>(
    state: &mut State<'input>
) -> Result<Absolute<'input>, Failure<'input>> {
    state.advance(|byte| byte == b'|')?;
    let expression = expression(state)?;
    state.advance(|byte| byte == b'|')?;
    return Ok(Absolute {
        expression: expression
    });
}

//> VALUE -> UNDEFINED
pub fn undefined<'input>(state: &mut State<'input>) -> Result<Undefined, Failure<'input>> {
    state.advance(|byte| byte == b'?')?;
    return Ok(Undefined);
}

//> VALUE -> CALL
pub fn call<'input>(state: &mut State<'input>) -> Result<Call<'input>, Failure<'input>> {
    let identifier = identifier(state)?;
    state.advance(|byte| byte == b'(')?;
    let with = state.optional(|state| {
        let first = expression(state)?;
        let mut rest = state.multiple(|state| {
            state.advance(|byte| byte == b',')?;
            expression(state)
        });
        rest.insert(0, first);
        Ok(rest)
    }).unwrap_or_default();
    state.advance(|byte| byte == b')')?;
    return Ok(Call {
        identifier: identifier,
        with: with
    });
}