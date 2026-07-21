//^
//^ HEAD
//^

//> HEAD -> MATHSYS
use mathsys::Interpreter;

//> HEAD -> STD
use std::sync::LazyLock;

//> HEAD -> SUPER
use super::resolver::resolve;

//> HEAD -> LIBUTILS
use libutils::{
    systemstd::System,
    systemio::Dump
};


//^
//^ INTERPRETER
//^

//> INTERPRETER -> STATIC
pub static INTERPRETER: LazyLock<Interpreter<'static>> = LazyLock::new(|| Interpreter {
    resolver: resolve,
    systemio: System::dump()
});