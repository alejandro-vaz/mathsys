//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::{
    solver::{
        context::Context,
        types::{
            NonTerminal,
            Item,
            Spawn
        },
        start::Start,
        level1::{
            Definition,
            Equation,
            Use,
            Node,
            Function
        },
        level2::Expression,
        level3::Term,
        level4::{
            Factor,
            Limit
        },
        level5::{
            Infinite,
            Variable,
            Nest,
            Vector,
            Whole,
            Absolute,
            Undefined,
            Rational,
            Call
        }
    },
    Interpreter,
    Resolver
};

//> HEAD -> STRUM_MACROS
use strum_macros::EnumString;

//> HEAD -> LIBUTILS
use libutils::active_reporting::Report;


//^
//^ TYPES
//^

//> TYPES -> OBJECT
#[derive(EnumString, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Object {
    Start,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Definition,
    Function,
    Node,
    Equation,
    Use,
    Expression,
    Term,
    Factor,
    Limit,
    Infinite,
    Variable,
    Nest,
    Vector,
    Whole,
    Absolute,
    Undefined,
    Rational,
    Call
} impl Object {
    pub fn summon<'valid>(
        &self, 
        mut children: Vec<Item<'valid>>, 
        context: &mut Context<'valid>, 
        mut report: Report<"">, 
        interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {return match self {
        Object::Start => Start::spawn(
            children, 
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Definition => Definition::spawn(
            children, 
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Function => Function::spawn(
            children, 
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Node => Node::spawn(
            children,
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Equation => Equation::spawn(
            children,
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Use => Use::spawn(
            children, 
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Expression => Expression::spawn(
            children, 
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Term => Term::spawn(
            children, 
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Factor => Factor::spawn(
            children, 
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Limit => Limit::spawn(
            children, 
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Infinite => Infinite::spawn(
            children, 
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Variable => Variable::spawn(
            children, 
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Nest => Nest::spawn(
            children, 
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Vector => Vector::spawn(
            children, 
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Whole => Whole::spawn(
            children, 
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Absolute => Absolute::spawn(
            children, 
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Undefined => Undefined::spawn(
            children, 
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Rational => Rational::spawn(
            children, 
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        Object::Call => Call::spawn(
            children, 
            context, 
            report.to(), 
            interpreter, 
            filename
        ),
        _ => Some(children.pop().unwrap().into_non_terminal().unwrap()),
    }}
}



