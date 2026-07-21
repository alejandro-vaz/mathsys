//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::{
    solver::{
        context::Context,
        spawn::Spawn,
        item::Item,
        nonterminal::NonTerminal
    },
    syntax::{
        Start,
        level1::{
            Definition,
            Equation,
            Function,
            Node,
            Use
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
            Number,
            Absolute,
            Undefined,
            Call
        }
    },
    failure::Failure
};

//> HEAD -> STRUM_MACROS
use strum_macros::EnumString;

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::Report,
    systemio::SystemIO
};


//^
//^ OBJECT
//^

//> OBJECT -> ENUM
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
    Number,
    Absolute,
    Undefined,
    Call
} 

//> OBJECT -> SUMMON
impl Object {
    pub fn summon<'valid>(
        &self, 
        mut children: Vec<Item<'valid>>, 
        context: &mut Context<'valid>, 
        mut report: Report<"">, 
        systemio: &'valid SystemIO<Failure<'valid>>,
        resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        filename: &'valid str
    ) -> NonTerminal<'valid> {return match self {
        Object::Start => Start::spawn(
            children, 
            context, 
            report.to(), 
            systemio, 
            resolver,
            filename
        ),
        Object::Definition => Definition::spawn(
            children, 
            context, 
            report.to(), 
            systemio, 
            resolver,
            filename
        ),
        Object::Function => Function::spawn(
            children, 
            context, 
            report.to(), 
            systemio, 
            resolver,
            filename
        ),
        Object::Node => Node::spawn(
            children,
            context, 
            report.to(), 
            systemio, 
            resolver,
            filename
        ),
        Object::Equation => Equation::spawn(
            children,
            context, 
            report.to(), 
            systemio, 
            resolver,
            filename
        ),
        Object::Use => Use::spawn(
            children, 
            context, 
            report.to(), 
            systemio, 
            resolver,
            filename
        ),
        Object::Expression => Expression::spawn(
            children, 
            context, 
            report.to(), 
            systemio, 
            resolver,
            filename
        ),
        Object::Term => Term::spawn(
            children, 
            context, 
            report.to(), 
            systemio, 
            resolver,
            filename
        ),
        Object::Factor => Factor::spawn(
            children, 
            context, 
            report.to(), 
            systemio, 
            resolver,
            filename
        ),
        Object::Limit => Limit::spawn(
            children, 
            context, 
            report.to(), 
            systemio, 
            resolver,
            filename
        ),
        Object::Infinite => Infinite::spawn(
            children, 
            context, 
            report.to(), 
            systemio, 
            resolver,
            filename
        ),
        Object::Variable => Variable::spawn(
            children, 
            context, 
            report.to(), 
            systemio, 
            resolver,
            filename
        ),
        Object::Nest => Nest::spawn(
            children, 
            context, 
            report.to(), 
            systemio, 
            resolver,
            filename
        ),
        Object::Vector => Vector::spawn(
            children, 
            context, 
            report.to(), 
            systemio, 
            resolver,
            filename
        ),
        Object::Number => Number::spawn(
            children, 
            context, 
            report.to(), 
            systemio, 
            resolver,
            filename
        ),
        Object::Absolute => Absolute::spawn(
            children, 
            context, 
            report.to(), 
            systemio, 
            resolver,
            filename
        ),
        Object::Undefined => Undefined::spawn(
            children, 
            context, 
            report.to(), 
            systemio, 
            resolver,
            filename
        ),
        Object::Call => Call::spawn(
            children, 
            context, 
            report.to(), 
            systemio,
            resolver, 
            filename
        ),
        _ => children.pop().unwrap().into_non_terminal().unwrap(),
    }}
}