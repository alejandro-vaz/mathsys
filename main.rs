//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(default_field_values)]
#![feature(const_trait_impl)]

//> HEAD -> MODULES
mod interfaceerror;
mod interpreter;
mod resolver;

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::Root,
    systemstd::{
        System,
        Argument
    }
};

//> HEAD -> INTERPRETER
use interpreter::INTERPRETER;

//> HEAD -> INTERFACEERROR
use interfaceerror::InterfaceError;


//^
//^ MAIN
//^

//> MAIN -> FUNCTION
fn main() -> () {
    let mut root = Root::default();
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
            INTERPRETER.latex(file.to_str().unwrap(), root.to())
        },
        name => System::critical(InterfaceError::UnknownTarget {
            name: name
        }, &*root)
    });
}