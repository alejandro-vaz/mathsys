//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::{
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
    solver::{
        item::Item,
        spawn::Spawn,
        context::Context,
        nonterminal::NonTerminal
    }
};

//> HEAD -> STRUM_MACROS
use strum_macros::{
    EnumString,
    EnumIter
};


//^
//^ OBJECT
//^

//> OBJECT -> ENUM
#[derive(EnumString, EnumIter, PartialEq, Eq, Hash, Debug)] // rm debug
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

//> OBJECT -> IMPLEMENTATION
impl Object {
    pub fn summon<'valid>(
        &self, 
        mut children: Vec<Item<'_, 'valid>>, 
        context: &mut Context<'valid>, 
        filename: &'valid str
    ) -> NonTerminal<'valid> {return match self {
        Object::Start => Start::spawn(children, context, filename),
        Object::Definition => Definition::spawn(children, context, filename),
        Object::Function => Function::spawn(children, context, filename),
        Object::Node => Node::spawn(children,context, filename),
        Object::Equation => Equation::spawn(children,context, filename),
        Object::Use => Use::spawn(children, context, filename),
        Object::Expression => Expression::spawn(children, context, filename),
        Object::Term => Term::spawn(children, context, filename),
        Object::Factor => Factor::spawn(children, context, filename),
        Object::Limit => Limit::spawn(children, context, filename),
        Object::Infinite => Infinite::spawn(children, context, filename),
        Object::Variable => Variable::spawn(children, context, filename),
        Object::Nest => Nest::spawn(children, context, filename),
        Object::Vector => Vector::spawn(children, context, filename),
        Object::Number => Number::spawn(children, context, filename),
        Object::Absolute => Absolute::spawn(children, context, filename),
        Object::Undefined => Undefined::spawn(children, context, filename),
        Object::Call => Call::spawn(children, context, filename),
        _ => children.pop().unwrap().into_non_terminal().unwrap(),
    }}
}