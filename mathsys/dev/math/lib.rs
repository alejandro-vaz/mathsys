//^
//^ HEAD
//^

//> HEAD -> FLAGS
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![feature(linkage)]
#![feature(try_trait_v2)]
#![feature(never_type)]

//> HEAD -> PRELUDE
mod prelude;
use prelude::{
    login,
    Reparser,
    Runtime,
    crash,
    Code,
    Instant
};

//> HEAD -> CONTEXT
mod context {
    pub mod infinite;
    pub mod integer;
    pub mod natural;
    pub mod nexists;
    pub mod rational;
    pub mod tensor;
    pub mod undefined;
    pub mod variable;
    pub mod whole;
}

//> HEAD -> DATA
mod data {
    pub mod _absolute;
    pub mod _annotation;
    pub mod _comment;
    pub mod _declaration;
    pub mod _definition;
    pub mod _equation;
    pub mod _expression;
    pub mod _factor;
    pub mod _infinite;
    pub mod _limit;
    pub mod _nest;
    pub mod _node;
    pub mod _start;
    pub mod _tensor;
    pub mod _term;
    pub mod _use;
    pub mod _variable;
    pub mod _whole;
}

//> HEAD -> LIB
mod lib {
    pub mod class;
    pub mod group;
    pub mod object;
    pub mod pointer;
    pub mod reparser;
    pub mod runtime;
    pub mod sign;
    pub mod stack;
    pub mod stdout;
    pub mod tip;
    pub mod value;
}


//^
//^ GLOBALS
//^

//> GLOBALS -> SETTINGS STRUCT
pub struct Settings {
    pub ir: &'static [u8],
    pub version: &'static str
}


//^
//^ ENTRY
//^

//> RUNTIME -> NOW
pub static mut NOW: Option<Instant> = None;

//> RUNTIME -> FUNCTION
pub fn run(settings: Settings) -> () {
    unsafe {NOW = Some(Instant::now())};
    login(settings.ir, settings.version);
    let mut reparser = Reparser::new();
    let memory = reparser.run(settings.ir);
    let mut runtime = Runtime::new();
    runtime.start(&memory);
    crash(Code::Success);
}