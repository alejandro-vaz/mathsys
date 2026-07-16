//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(phantom_variance_markers)]

//> HEAD -> MODULES
mod interfaceerror;

//> HEAD -> MATHSYS
use mathsys::{
    Interpreter,
    Resolver
};

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::{
        Report,
         Root
    },
    systemstd::{
        System,
        Argument
    }
};

//> HEAD -> CORE
use core::marker::PhantomCovariantLifetime;

//> HEAD -> ELSA
use elsa::FrozenMap;

//> HEAD -> INTERFACEERROR
pub use interfaceerror::InterfaceError;


//^
//^ MAIN
//^

//> MAIN -> HANDLER
struct Handler<'valid> {
    cache: FrozenMap<&'valid str, String>
} impl<'valid> Resolver<'valid> for Handler<'valid> {
    fn resolve(&'valid self, filename: &'valid str, report: Report<"Resolver">) -> &'valid str {
        return match self.cache.get(filename) {
            Some(cached) => cached,
            None => self.cache.insert(
                filename,
                System::expect(System::expect(System::open(filename), &*report).read(), &*report)
            )
        };
    }
}

//> MAIN -> FUNCTION
fn main() -> () {
    let mut root = Root::default();
    let interpreter = Interpreter {
            resolver: Handler {
            cache: FrozenMap::new()
        },
        lifetime: PhantomCovariantLifetime::new()
    };
    let (target, arguments) = match System::arguments() {
        [Argument::Target(target), arguments @ ..] => (target, arguments),
        [Argument::Path(_), Argument::Target(target), arguments @ ..] => (target, arguments),
        _ => System::critical(InterfaceError::TargetNotProvided, &*root)
    };
    let output = match target.as_str() {
        "latex" => {
            let file = match arguments {
                [Argument::Path(file)] => file,
                _ => System::critical(InterfaceError::IncorrectLatexArguments, &*root)
            };
            interpreter.latex(file, root.to())
        },
        name => System::critical(InterfaceError::UnknownTarget {
            name: name
        }, &*root)
    };
    System::print(output);
}