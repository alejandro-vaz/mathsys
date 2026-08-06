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
    return state.advance(|token| token.is_infinite().then_some(Infinite));
}

//> VALUE -> IDENTIFIER
pub fn identifier<'input>(
    state: &mut State<'input>
) -> Result<Identifier<'input>, Failure<'input>> {
    return state.advance(|token| token.as_identifier().map(|identifier| Identifier {
        name: *identifier
    }));
}

//> VALUE -> NEST
pub fn nest<'input>(state: &mut State<'input>) -> Result<Nest<'input>, Failure<'input>> {
    state.advance(|token| token.is_open().then_some(()))?;
    let inside = state.optional(expression);
    return state.advance(|token| token.is_close().then_some(Nest {
        inside: inside
    }));
}

//> VALUE -> VECTOR
pub fn vector<'input>(
    state: &mut State<'input>
) -> Result<Vector<'input>, Failure<'input>> {
    state.advance(|token| token.is_enter().then_some(()))?;
    let expressions = state.optional(|state| {
        let first = expression(state)?;
        let mut rest = state.multiple(|state| {
            state.advance(|token| token.is_comma().then_some(()))?;
            expression(state)
        });
        rest.insert(0, first);
        Ok(rest)
    }).unwrap_or_default();
    return state.advance(|token| token.is_exit().then_some(Vector {
        expressions: expressions
    }));
}

//> VALUE -> NUMBER
pub fn number<'input>(
    state: &mut State<'input>
) -> Result<Number<'input>, Failure<'input>> {
    return state.advance(|token| token.as_number().map(|number| Number {
        number: *number
    }));
}

//> VALUE -> ABSOLUTE
pub fn absolute<'input>(
    state: &mut State<'input>
) -> Result<Absolute<'input>, Failure<'input>> {
    state.advance(|token| token.is_pipe().then_some(()))?;
    let expression = expression(state)?;
    return state.advance(|token| token.is_pipe().then_some(Absolute {
        expression: expression
    }));
}

//> VALUE -> UNDEFINED
pub fn undefined<'input>(state: &mut State<'input>) -> Result<Undefined, Failure<'input>> {
    return state.advance(|token| token.is_undefined().then_some(Undefined));
}

//> VALUE -> CALL
pub fn call<'input>(state: &mut State<'input>) -> Result<Call<'input>, Failure<'input>> {
    let identifier = identifier(state)?;
    state.advance(|token| token.is_open().then_some(()))?;
    let with = state.optional(|state| {
        let first = expression(state)?;
        let mut rest = state.multiple(|state| {
            state.advance(|token| token.is_comma().then_some(()))?;
            expression(state)
        });
        rest.insert(0, first);
        Ok(rest)
    }).unwrap_or_default();
    return state.advance(|token| token.is_close().then_some(Call {
        identifier: identifier,
        with: with
    }));
}