//^
//^ HEAD
//^

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> CRATE
use crate::latex::LaTeX;

//> HEAD -> SUPER
use super::{
    expression::Expression,
    value::Identifier
};


//^
//^ STATEMENT
//^

//> STATEMENT -> ENUM
#[enum_dispatch(LaTeX)]
pub enum Statement<'valid> {
    Definition(Definition<'valid>),
    Function(Function<'valid>),
    Node(Node<'valid>),
    Equation(Equation<'valid>),
    Use(Use<'valid>)
}

//> STATEMENT -> DEFINITION
pub struct Definition<'valid> {
    pub identifier: Identifier<'valid>,
    pub expression: Expression<'valid>
} 

//> STATEMENT -> FUNCTION
pub struct Function<'valid> {
    pub identifier: Identifier<'valid>,
    pub arguments: Vec<Identifier<'valid>>,
    pub expression: Expression<'valid>
} 

//> STATEMENT -> NODE
pub struct Node<'valid> {
    pub expression: Expression<'valid>
} 

//> STATEMENT -> EQUATION
pub struct Equation<'valid> {
    pub expressions: [Expression<'valid>; 2]
} 

//> STATEMENT -> USE
pub struct Use<'valid> {
    pub module: &'valid str
} 