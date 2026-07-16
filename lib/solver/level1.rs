//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    level2::Level2,
    level5::Variable,
    types::{
        Spawn,
        Item,
        NonTerminal
    },
    context::Context,
    start::Start
};

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> CRATE
use crate::{
    latex::LaTeX,
    tokenizer::tokenize,
    filter::filter,
    Interpreter,
    parser::parse,
    extensor::extend,
    solver::solve,
    Resolver,
    failure::Failure,
    reducer::reduce
};

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::Report,
    systemstd::System
};

//> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;


//^
//^ 1ºLEVEL
//^

//> 1ºLEVEL -> ENUM
#[enum_dispatch(LaTeX)]
#[derive(Clone, EnumAsInner, Debug)]
pub enum Level1<'valid> {
    Definition(Definition<'valid>),
    Function(Function<'valid>),
    Node(Node<'valid>),
    Equation(Equation<'valid>),
    Use(Use<'valid>)
}

//> 1ºLEVEL -> DEFINITION
#[derive(Clone, Debug)]
pub struct Definition<'valid> {
    pub variable: Variable<'valid>,
    pub value: Level2<'valid>
} impl<'valid> Spawn<'valid> for Definition<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>,
        _context: &mut Context<'valid>,
        _report: Report<"">,
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {return Some(NonTerminal::Level1(Level1::Definition(Self {
        variable: children.remove(0).into_non_terminal().unwrap().into_level5().unwrap().into_variable().unwrap(),
        value: children.pop().unwrap().into_non_terminal().unwrap().into_level2().unwrap()
    })))}
}

//> 1ºLEVEL -> FUNCTION
#[derive(Clone, Debug)]
pub struct Function<'valid> {
    pub variable: Variable<'valid>,
    pub arguments: Vec<Variable<'valid>>,
    pub expression: Level2<'valid>
} impl<'valid> Spawn<'valid> for Function<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>,
        context: &mut Context<'valid>,
        _report: Report<"">,
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {
        let variable = children.remove(0).into_non_terminal().unwrap().into_level5().unwrap().into_variable().unwrap();
        context.functions.insert(variable.name);
        let level2 = children.pop().unwrap().into_non_terminal().unwrap().into_level2().unwrap();
        return Some(NonTerminal::Level1(Level1::Function(Self {
            variable: variable,
            arguments: children.into_iter().map(|item| item.into_non_terminal().unwrap().into_level5().unwrap().into_variable().unwrap()).collect(),
            expression: level2
        })))
    }
}

//> 1ºLEVEL -> NODE
#[derive(Clone, Debug)]
pub struct Node<'valid> {
    pub value: Level2<'valid>
} impl<'valid> Spawn<'valid> for Node<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>,
        _context: &mut Context<'valid>,
        _report: Report<"">,
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {return Some(NonTerminal::Level1(Level1::Node(Self {
        value: children.pop().unwrap().into_non_terminal().unwrap().into_level2().unwrap()
    })))}
}

//> 1ºLEVEL -> EQUATION
#[derive(Clone, Debug)]
pub struct Equation<'valid> {
    pub left: Level2<'valid>,
    pub right: Level2<'valid>
} impl<'valid> Spawn<'valid> for Equation<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>,
        _context: &mut Context<'valid>,
        _report: Report<"">,
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {return Some(NonTerminal::Level1(Level1::Equation(Self {
        right: children.pop().unwrap().into_non_terminal().unwrap().into_level2().unwrap(),
        left: children.pop().unwrap().into_non_terminal().unwrap().into_level2().unwrap()
    })))}
}

//> 1ºLEVEL -> USE
#[derive(Clone, Debug)]
pub struct Use<'valid> {
    pub module: &'valid str,
    pub start: Start<'valid>
} impl<'valid> Spawn<'valid> for Use<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>,
        context: &mut Context<'valid>,
        mut report: Report<"">,
        interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {
        let module = children.pop().unwrap().into_token().unwrap().value.strip_circumfix('"', '"').unwrap();
        context.dependencies.entry(filename).or_default().insert(module);
        return if context.dependencies.entry(module).or_default().contains(filename) {
            System::critical(Failure::CircularImport {
                from: filename,
                to: module
            }, &*report);
        } else {Some(NonTerminal::Level1(Level1::Use(Self {
            module: module,
            start: solve(
                parse(
                    filter(tokenize(
                        interpreter.resolver.resolve(
                            module,
                            report.to()
                        ),
                        report.to()
                    )),
                    extend(reduce()),
                ),
                Some(context),
                module,
                interpreter,
                report.to()
            )
        })))};
    }
}