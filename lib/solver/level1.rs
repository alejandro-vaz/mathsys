//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    spawn::Spawn,
    item::Item,
    nonterminal::NonTerminal,
    context::Context
};

//> HEAD -> CRATE
use crate::{
    tokenizer::tokenize,
    filter::filter,
    parser::parse,
    solver::solve,
    failure::Failure,
    syntax::level1::{
        Level1,
        Definition,
        Function,
        Node,
        Equation,
        Use
    }
};

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::Report,
    systemio::SystemIO
};


//^
//^ 1ºLEVEL
//^

//> 1ºLEVEL -> DEFINITION
impl<'valid> Spawn<'valid> for Definition<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>,
        _context: &mut Context<'valid>,
        _report: Report<"">,
        _systemio: &'valid SystemIO<Failure<'valid>>,
        _resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        _filename: &'valid str
    ) -> NonTerminal<'valid> {
        return NonTerminal::Level1(Level1::Definition(Self {
            variable: children.remove(0).into_non_terminal().unwrap().into_level5().unwrap().into_variable().unwrap(),
            value: children.pop().unwrap().into_non_terminal().unwrap().into_level2().unwrap()
        }));
    }
}

//> 1ºLEVEL -> FUNCTION
impl<'valid> Spawn<'valid> for Function<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>,
        context: &mut Context<'valid>,
        _report: Report<"">,
        _systemio: &'valid SystemIO<Failure<'valid>>,
        _resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        _filename: &'valid str
    ) -> NonTerminal<'valid> {
        let variable = children.remove(0).into_non_terminal().unwrap().into_level5().unwrap().into_variable().unwrap();
        context.functions.insert(variable.name);
        let level2 = children.pop().unwrap().into_non_terminal().unwrap().into_level2().unwrap();
        return NonTerminal::Level1(Level1::Function(Self {
            variable: variable,
            arguments: children.into_iter().map(|item| item.into_non_terminal().unwrap().into_level5().unwrap().into_variable().unwrap()).collect(),
            expression: level2
        }));
    }
}

//> 1ºLEVEL -> NODE
impl<'valid> Spawn<'valid> for Node<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>,
        _context: &mut Context<'valid>,
        _report: Report<"">,
        _systemio: &'valid SystemIO<Failure<'valid>>,
        _resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        _filename: &'valid str
    ) -> NonTerminal<'valid> {return NonTerminal::Level1(Level1::Node(Self {
        value: children.pop().unwrap().into_non_terminal().unwrap().into_level2().unwrap()
    }))}
}

//> 1ºLEVEL -> EQUATION
impl<'valid> Spawn<'valid> for Equation<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>,
        _context: &mut Context<'valid>,
        _report: Report<"">,
        _systemio: &'valid SystemIO<Failure<'valid>>,
        _resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        _filename: &'valid str
    ) -> NonTerminal<'valid> {return NonTerminal::Level1(Level1::Equation(Self {
        right: children.pop().unwrap().into_non_terminal().unwrap().into_level2().unwrap(),
        left: children.pop().unwrap().into_non_terminal().unwrap().into_level2().unwrap()
    }))}
}

//> 1ºLEVEL -> USE
impl<'valid> Spawn<'valid> for Use<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>,
        context: &mut Context<'valid>,
        mut report: Report<"">,
        systemio: &'valid SystemIO<Failure<'valid>>,
        resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        filename: &'valid str
    ) -> NonTerminal<'valid> {
        let module = children.pop().unwrap().into_token().unwrap().into_module().unwrap().strip_circumfix('"', '"').unwrap();
        context.dependencies.entry(filename).or_default().insert(module);
        return if context.dependencies.entry(module).or_default().contains(filename) {
            (systemio.critical)(Failure::CircularImport { 
                from: filename, 
                to: module 
            }, &*report);
        } else {NonTerminal::Level1(Level1::Use(Self {
            _module: module,
            _start: solve(
                parse(filter(tokenize(
                    resolver(
                        module,
                        report.to()
                    ),
                    filename,
                    &systemio,
                    report.to()
                ))),
                context,
                module,
                systemio,
                resolver,
                report.to()
            )
        }))};
    }
}