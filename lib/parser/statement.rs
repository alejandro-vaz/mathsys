//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    state::State,
    expression::expression,
    value::identifier
};

//> HEAD -> CRATE
use crate::{
    syntax::statement::{
        Statement,
        Definition,
        Function,
        Node,
        Equation
    },
    failure::Failure
};


//^
//^ STATEMENT
//^

//> STATEMENT -> DISPATCH
pub fn statement<'input>(
    state: &mut State<'input>
) -> Result<Statement<'input>, Failure<'input>> {return match state.optional(definition) {
    None => match state.optional(function) {
        None => match state.optional(node) {
            None => match state.optional(equation) {
                Some(equation) => Ok(Statement::Equation(equation)),
                None => Err(Failure::CouldntParseStatement)
            },
            Some(node) => Ok(Statement::Node(node))
        },
        Some(function) => Ok(Statement::Function(function))
    },
    Some(definition) => Ok(Statement::Definition(definition))
}}

//> STATEMENT -> DEFINITION
pub fn definition<'input>(
    state: &mut State<'input>
) -> Result<Definition<'input>, Failure<'input>> {
    let identifier = identifier(state)?;
    state.advance(|byte| byte == b':')?;
    state.advance(|byte| byte == b'=')?;
    return Ok(Definition {
        identifier: identifier,
        expression: expression(state)?
    });
}

//> STATEMENT -> FUNCTION
pub fn function<'input>(
    state: &mut State<'input>
) -> Result<Function<'input>, Failure<'input>> {
    let name = identifier(state)?;
    state.advance(|byte| byte == b'(')?;
    let arguments = state.optional(|state| {
        let first = identifier(state)?;
        let mut rest = state.multiple(|state| {
            state.advance(|byte| byte == b',')?;
            identifier(state)
        });
        rest.insert(0, first);
        Ok(rest)
    }).unwrap_or_default();
    state.advance(|byte| byte == b')')?;
    state.advance(|byte| byte == b':')?;
    state.advance(|byte| byte == b'=')?;
    return Ok(Function {
        identifier: name,
        arguments: arguments,
        expression: expression(state)?
    })
}

//> STATEMENT -> NODE
pub fn node<'input>(
    state: &mut State<'input>
) -> Result<Node<'input>, Failure<'input>> {
    return Ok(Node {
        expression: expression(state)?
    });
}

//> STATEMENT -> EQUATION
pub fn equation<'input>(
    state: &mut State<'input>
) -> Result<Equation<'input>, Failure<'input>> {
    let left = expression(state)?;
    state.advance(|byte| byte == b'=')?;
    return Ok(Equation {
        expressions: [left, expression(state)?]
    });
}