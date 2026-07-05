//^
//^ HEAD
//^

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> SUPER
use super::{
    level2::Level2,
    types::{
        Spawn,
        Item,
        NonTerminal
    },
    context::Context
};

//> HEAD -> CRATE
use crate::{
    latex::LaTeX,
    Interpreter,
    Resolver
};

//> HEAD -> LIBUTILS
use libutils::report::{
    Report,
    Same
};

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;


//^
//^ 5ºLEVEL
//^

//> 5ºLEVEL -> ENUM
#[enum_dispatch(LaTeX)]
#[derive(Clone, EnumAsInner, Debug)]
pub enum Level5<'valid> {
    Infinite,
    Variable(Variable<'valid>),
    Nest(Nest<'valid>),
    Vector(Vector<'valid>),
    Whole(Whole<'valid>),
    Absolute(Absolute<'valid>),
    Undefined,
    Rational(Rational<'valid>),
    Call(Call<'valid>)
}

//> 5ºLEVEL -> INFINITE
#[derive(Clone, Debug)]
pub struct Infinite; impl<'valid> Spawn<'valid> for Infinite {
    fn spawn(
        _children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<Same>, 
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {return Some(NonTerminal::Level5(Level5::Infinite(Self)))}
}

//> 5ºLEVEL -> VARIABLE
#[derive(Clone, Debug)]
pub struct Variable<'valid> {
    pub name: &'valid str
} impl<'valid> Spawn<'valid> for Variable<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<Same>, 
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {return Some(NonTerminal::Level5(Level5::Variable(Self {
        name: children.remove(0).into_token().unwrap().value
    })))}
}

//> 5ºLEVEL -> NEST
#[derive(Clone, Debug)]
pub struct Nest<'valid> {
    pub value: Option<Level2<'valid>>
} impl<'valid> Spawn<'valid> for Nest<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<Same>, 
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {return Some(NonTerminal::Level5(Level5::Nest(Self {
        value: children.pop().map(|item| item.into_non_terminal().unwrap().into_level2().unwrap())
    })))}
}

//> 5ºLEVEL -> VECTOR
#[derive(Clone, Debug)]
pub struct Vector<'valid> {
    pub values: Vec<Level2<'valid>>
} impl<'valid> Spawn<'valid> for Vector<'valid> {
    fn spawn(
        children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<Same>, 
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {return Some(NonTerminal::Level5(Level5::Vector(Self {
        values: children.into_iter().map(|item| item.into_non_terminal().unwrap().into_level2().unwrap()).collect()
    })))}
}

//> 5ºLEVEL -> WHOLE
#[derive(Clone, Debug)]
pub struct Whole<'valid> {
    pub number: &'valid str
} impl<'valid> Spawn<'valid> for Whole<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<Same>, 
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {return Some(NonTerminal::Level5(Level5::Whole(Self {
        number: children.remove(0).into_token().unwrap().value
    })))}
}

//> 5ºLEVEL -> ABSOLUTE
#[derive(Clone, Debug)]
pub struct Absolute<'valid> {
    pub value: Level2<'valid>
} impl<'valid> Spawn<'valid> for Absolute<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>,
        _context: &mut Context<'valid>, 
        _report: Report<Same>, 
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {return Some(NonTerminal::Level5(Level5::Absolute(Self {
        value: children.remove(0).into_non_terminal().unwrap().into_level2().unwrap()
    })))}
}

//> 5ºLEVEL -> UNDEFINED
#[derive(Clone, Debug)]
pub struct Undefined; impl<'valid> Spawn<'valid> for Undefined {
    fn spawn(
        _children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<Same>, 
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {return Some(NonTerminal::Level5(Level5::Undefined(Self)))}
}

//> 5ºLEVEL -> RATIONAL
#[derive(Clone, Debug)]
pub struct Rational<'valid> {
    pub number: &'valid str
} impl<'valid> Spawn<'valid> for Rational<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<Same>, 
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {return Some(NonTerminal::Level5(Level5::Rational(Self {
        number: children.remove(0).into_token().unwrap().value
    })))}
}

//> 5ºLEVEL -> CALL
#[derive(Clone, Debug)]
pub struct Call<'valid> {
    pub to: Variable<'valid>,
    pub with: Vec<Level2<'valid>>
} impl<'valid> Spawn<'valid> for Call<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<Same>, 
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {return Some(NonTerminal::Level5(Level5::Call(Self {
        to: children.remove(0).into_non_terminal().unwrap().into_level5().unwrap().into_variable().unwrap(),
        with: children.into_iter().map(|item| item.into_non_terminal().unwrap().into_level2().unwrap()).collect()
    })))}
}