//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(phantom_variance_markers)]
#![feature(default_field_values)]

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
        Argument,
        OpenMode
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
    fn resolve(
        &'valid self, 
        filename: &'valid str, 
        report: Report<"Resolver">
    ) -> &'valid str {return match self.cache.get(filename) {
        Some(cached) => cached,
        None => self.cache.insert(
            filename,
            System::expect(System::expect(
                System::path(filename).file::<{OpenMode::Read}>(None), 
                &*report
            ).read(), &*report)
        )
    }}
}

//> MAIN -> FUNCTION
fn main() -> () {
    let mut root = Root::default();
    let interpreter = Interpreter {
        resolver: Handler {
            cache: FrozenMap::new()
        },
        lifetime: PhantomCovariantLifetime::new(),
        warning: |failure, chain| System::warning(failure, chain),
        error: |failure, chain| System::error(failure, chain),
        critical: |failure, chain| System::critical(failure, chain)
    };
    let (target, arguments) = match System::arguments() {
        [Argument::Target {to}, arguments @ ..] => (to, arguments),
        [Argument::Path {..}, Argument::Target {to}, arguments @ ..] => (to, arguments),
        _ => System::critical(InterfaceError::TargetNotProvided, &*root)
    };
    System::print(match target.as_str() {
        "latex" => {
            let file = match arguments {
                [Argument::Path {buffer}] => buffer,
                _ => System::critical(InterfaceError::IncorrectLatexArguments, &*root)
            };
            interpreter.latex(file.to_str().unwrap(), root.to())
        },
        name => System::critical(InterfaceError::UnknownTarget {
            name: name
        }, &*root)
    });
}