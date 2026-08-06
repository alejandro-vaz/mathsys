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
        Equation,
        Use
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
                None => match state.optional(r#use) {
                    Some(r#use) => Ok(Statement::Use(r#use)),
                    None => Err(Failure::CouldntParseStatement)
                },
                Some(equation) => Ok(Statement::Equation(equation)),
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
    state.advance(|token| token.is_definition().then_some(()))?;
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
    state.advance(|token| token.is_open().then_some(()))?;
    let arguments = state.optional(|state| {
        let first = identifier(state)?;
        let mut rest = state.multiple(|state| {
            state.advance(|token| token.is_comma().then_some(()))?;
            identifier(state)
        });
        rest.insert(0, first);
        Ok(rest)
    }).unwrap_or_default();
    state.advance(|token| token.is_close().then_some(()))?;
    state.advance(|token| token.is_definition().then_some(()))?;
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
    state.advance(|token| token.is_equality().then_some(()))?;
    return Ok(Equation {
        expressions: [left, expression(state)?]
    });
}

//> STATEMENT -> USE
pub fn r#use<'input>(
    state: &mut State<'input>
) -> Result<Use<'input>, Failure<'input>> {
    state.advance(|token| token.is_use().then_some(()))?;
    let module = state.advance(|token| token.as_module().map(|module| *module))?;
    return Ok(Use {
        module: module,
    })
}