//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(try_blocks)]
#![feature(phantom_variance_markers)]

//> HEAD -> MATHSYS
use mathsys::{
    Interpreter,
    Failure,
    Resolver
};

//> HEAD -> LIBUTILS
use libutils::{
    report::{
        Report,
        Name
    },
    terminal::TERMINAL,
    console::{
        Argument,
        Console,
        Synchronization,
        Descriptor
    }
};

//> HEAD -> ELSA
use elsa::FrozenMap;

//> HEAD -> CORE
use core::marker::PhantomCovariantLifetime;


//^
//^ MAIN
//^

//> MAIN -> HANDLER
struct Handler<'valid> {
    cache: FrozenMap<&'valid str, String>
} impl<'valid> Resolver<'valid> for Handler<'valid> {
    fn resolve(&'valid self, filename: &'valid str, report: Report<Name<'_, "Resolver">>) -> Option<&'valid str> {
        return Some(match self.cache.get(filename) {
            Some(cached) => cached,
            None => self.cache.insert(
                filename, 
                report.apply(report.apply(TERMINAL.open(filename)).value?.read()).value?
            )
        });
    }
}

//> MAIN -> FUNCTION
fn main() -> () {try {
    let mut report = Report::default();
    let interpreter = Interpreter {
        resolver: Handler {
            cache: FrozenMap::new()
        },
        lifetime: PhantomCovariantLifetime::new()
    };
    let file = match TERMINAL.arguments() {
        [Argument::Path(_), Argument::Path(file)] => file,
        [_, _] => report.issue(Failure::IncorrectArgumentDistribution).none()?,
        arguments => report.issue(Failure::IncorrectArgumentAmount(arguments.len())).none()?
    };
    let latex = interpreter.latex(file, report.to())?;
    TERMINAL.print(&latex).sync();
};}