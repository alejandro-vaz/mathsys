//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    dispatch
};

//> HEAD -> LOCAL
use super::{
    context::Context,
    super::{
        backends::Spawn,
        tokenizer::token::BindedToken,
        syntax::{
            Start,
            level1::{
                Level1,
                Definition,
                Function,
                Node, 
                Equation, 
                Use
            },
            level2::{
                Level2,
                Expression
            },
            level3::{
                Level3,
                Term
            },
            level4::{
                Level4,
                Factor,
                Limit
            },
            level5::{
                Level5,
                Absolute, 
                Infinite, 
                Nest, 
                Rational, 
                Tensor, 
                Undefined, 
                Variable, 
                Whole,
                Call
            }
        }
    }
};


//^
//^ NONTERMINAL
//^

//> NONTERMINAL -> TRAIT
#[dispatch(Backends)]
#[derive(Debug, Clone)]
pub(crate) enum NonTerminal {
    Start(Start),
    Level1(Level1),
    Level2(Level2),
    Level3(Level3),
    Level4(Level4),
    Level5(Level5)
}

//> NONTERMINAL -> OBJECT
#[derive(Eq, Hash, PartialEq, Clone)]
pub(crate) enum Object {
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
    Tensor,
    Whole,
    Absolute,
    Undefined,
    Rational,
    Call
} impl Object {
    pub(super) fn summon<'parsing>(self, items: Vec<Item>, context: Option<&mut Context>) -> NonTerminal {return match self {
        Object::Start => Start::spawn(items, context),
        Object::Level1 => if let Item::NonTerminal(nonterminal @ NonTerminal::Level1(_)) = items.into_iter().next().unwrap() {nonterminal} else {panic!()},
        Object::Level2 => if let Item::NonTerminal(nonterminal @ NonTerminal::Level2(_)) = items.into_iter().next().unwrap() {nonterminal} else {panic!()},
        Object::Level3 => if let Item::NonTerminal(nonterminal @ NonTerminal::Level3(_)) = items.into_iter().next().unwrap() {nonterminal} else {panic!()},
        Object::Level4 => if let Item::NonTerminal(nonterminal @ NonTerminal::Level4(_)) = items.into_iter().next().unwrap() {nonterminal} else {panic!()},
        Object::Level5 => if let Item::NonTerminal(nonterminal @ NonTerminal::Level5(_)) = items.into_iter().next().unwrap() {nonterminal} else {panic!()},
        Object::Definition => Definition::spawn(items, context),
        Object::Function => Function::spawn(items, context),
        Object::Node => Node::spawn(items, context),
        Object::Equation => Equation::spawn(items, context),
        Object::Use => Use::spawn(items, context),
        Object::Expression => Expression::spawn(items, context),
        Object::Term => Term::spawn(items, context),
        Object::Factor => Factor::spawn(items, context),
        Object::Limit => Limit::spawn(items, context),
        Object::Infinite => Infinite::spawn(items, context),
        Object::Variable => Variable::spawn(items, context),
        Object::Nest => Nest::spawn(items, context),
        Object::Tensor => Tensor::spawn(items, context),
        Object::Whole => Whole::spawn(items, context),
        Object::Absolute => Absolute::spawn(items, context),
        Object::Undefined => Undefined::spawn(items, context),
        Object::Rational => Rational::spawn(items, context),
        Object::Call => Call::spawn(items, context)
    }}
}


//^
//^ TEMPORAL
//^

//> TEMPORAL -> ITEM
#[derive(Clone, Debug)]
pub(crate) enum Item<'resolving> {
    NonTerminal(NonTerminal),
    Token(BindedToken<'resolving>)
}

//> TEMPORAL -> PARTITION
#[derive(Clone)]
pub(super) enum Partition<'resolving> {
    NonTerminal(NonTerminal),
    Internal(Vec<Item<'resolving>>),
    Token(BindedToken<'resolving>)
}