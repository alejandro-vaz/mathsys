//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    dispatch
};

//> HEAD -> LOCAL
use super::{
    super::{
        backends::Spawn,
        tokenizer::token::BindedToken,
        Settings,
        Issue,
        Context,
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
#[dispatch(Backends, Contextualize)]
#[derive(Debug, Clone)]
pub(crate) enum NonTerminal {
    Start,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5
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
    pub(super) fn summon<'parsing>(&self, items: Vec<Item>, settings: &Settings, context: &mut Context) -> Result<NonTerminal, Issue> {return match self {
        Object::Start => Start::spawn(items, settings, context),
        Object::Level1 => if let Item::NonTerminal(nonterminal @ NonTerminal::Level1(_)) = items.into_iter().next().unwrap() {Ok(nonterminal)} else {panic!()},
        Object::Level2 => if let Item::NonTerminal(nonterminal @ NonTerminal::Level2(_)) = items.into_iter().next().unwrap() {Ok(nonterminal)} else {panic!()},
        Object::Level3 => if let Item::NonTerminal(nonterminal @ NonTerminal::Level3(_)) = items.into_iter().next().unwrap() {Ok(nonterminal)} else {panic!()},
        Object::Level4 => if let Item::NonTerminal(nonterminal @ NonTerminal::Level4(_)) = items.into_iter().next().unwrap() {Ok(nonterminal)} else {panic!()},
        Object::Level5 => if let Item::NonTerminal(nonterminal @ NonTerminal::Level5(_)) = items.into_iter().next().unwrap() {Ok(nonterminal)} else {panic!()},
        Object::Definition => Definition::spawn(items, settings, context),
        Object::Function => Function::spawn(items, settings, context),
        Object::Node => Node::spawn(items, settings, context),
        Object::Equation => Equation::spawn(items, settings, context),
        Object::Use => Use::spawn(items, settings, context),
        Object::Expression => Expression::spawn(items, settings, context),
        Object::Term => Term::spawn(items, settings, context),
        Object::Factor => Factor::spawn(items, settings, context),
        Object::Limit => Limit::spawn(items, settings, context),
        Object::Infinite => Infinite::spawn(items, settings, context),
        Object::Variable => Variable::spawn(items, settings, context),
        Object::Nest => Nest::spawn(items, settings, context),
        Object::Tensor => Tensor::spawn(items, settings, context),
        Object::Whole => Whole::spawn(items, settings, context),
        Object::Absolute => Absolute::spawn(items, settings, context),
        Object::Undefined => Undefined::spawn(items, settings, context),
        Object::Rational => Rational::spawn(items, settings, context),
        Object::Call => Call::spawn(items, settings, context)
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