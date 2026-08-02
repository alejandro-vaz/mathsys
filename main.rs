//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(default_field_values)]
#![feature(const_trait_impl)]
#![feature(phantom_variance_markers)]

//> HEAD -> MODULES
mod handler;
mod interfaceerror;

//> HEAD -> LIBUTILS
use libutils::systemstd::{
    System,
    Argument
};

//> HEAD -> INTERFACEERROR
use interfaceerror::InterfaceError;

//> HEAD -> MATHSYS
use mathsys::Interpreter;

//> HEAD -> HANDLER
use handler::Handler;

//> HEAD -> CORE
use core::marker::PhantomCovariantLifetime;


//^
//^ MAIN
//^

//> MAIN -> FUNCTION
fn main() -> () {
    let interpreter = Interpreter {
        runtime: Handler::default(),
        lifetime: PhantomCovariantLifetime::new()
    };
    let (target, arguments) = match System::arguments() {
        [Argument::Target {to}, arguments @ ..] => (to, arguments),
        [Argument::Path {..}, Argument::Target {to}, arguments @ ..] => (to, arguments),
        _ => System::critical(InterfaceError::TargetNotProvided)
    };
    System::print(match target.as_str() {
        "latex" => {
            let file = match arguments {
                [Argument::Path {buffer}] => buffer,
                _ => System::critical(InterfaceError::IncorrectLatexArguments)
            };
            interpreter.latex(file.to_str().unwrap());
            String::from("yessss")
        },
        name => System::critical(InterfaceError::UnknownTarget {
            name: name
        })
    });
}