//^
//^ HEAD
//^

//> HEAD -> FLAGS
#![allow(unused_variables)]
#![allow(nonstandard_style)]
#![feature(linkage)]
#![feature(try_trait_v2)]
#![feature(never_type)]
#![allow(static_mut_refs)]

//> HEAD -> PRELUDE
mod prelude;
use prelude::{
    stdout,
    Reparser,
    Runtime,
    Code
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
    pub mod _casts;
    pub mod _declaration;
    pub mod _definition;
    pub mod _equation;
    pub mod _expression;
    pub mod _factor;
    pub mod _infinite;
    pub mod _limit;
    pub mod _nest;
    pub mod _node;
    pub mod _rational;
    pub mod _start;
    pub mod _tensor;
    pub mod _term;
    pub mod _undefined;
    pub mod _use;
    pub mod _variable;
    pub mod _whole;
}

//> HEAD -> LIB
mod lib {
    pub mod class;
    pub mod group;
    pub mod object;
    pub mod opcode;
    pub mod pointer;
    pub mod reparser;
    pub mod runtime;
    pub mod sign;
    pub mod stack;
    pub mod stdout;
    pub mod tip;
}


//^
//^ GLOBALS
//^

//> GLOBALS -> SETTINGS STRUCT
pub struct Settings {
    pub ir: &'static [u8],
    pub version: &'static str,
    pub debug: bool,
    pub class: bool,
    pub chore: bool,
    pub trace: bool,
    pub alert: bool,
    pub point: bool
}


//^
//^ ENTRY
//^


//> RUNTIME -> FUNCTION
pub fn run(settings: Settings) -> () {
    stdout.init(&settings);
    let mut reparser = Reparser::new();
    let memory = reparser.run(settings.ir);
    let mut runtime = Runtime::new();
    runtime.start(&memory);
    stdout.crash(Code::Success);
}