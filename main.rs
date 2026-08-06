//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(default_field_values)]
#![feature(const_trait_impl)]

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


//^
//^ MAIN
//^

//> MAIN -> FUNCTION
fn main() -> () {
    let interpreter = Interpreter::from(Handler::default());
    let (target, arguments) = match System::arguments() {
        [Argument::Target {to}, arguments @ ..] => (to, arguments),
        [Argument::Path {..}, Argument::Target {to}, arguments @ ..] => (to, arguments),
        _ => System::critical([InterfaceError::TargetNotProvided])
    };
    System::print(match target.as_str() {
        "latex" => {
            let file = match arguments {
                [Argument::Path {buffer}] => buffer,
                _ => System::critical([InterfaceError::IncorrectLatexArguments])
            };
            interpreter.latex(file.to_str().unwrap())
        },
        name => System::critical([InterfaceError::UnknownTarget {
            name: name
        }])
    });
}