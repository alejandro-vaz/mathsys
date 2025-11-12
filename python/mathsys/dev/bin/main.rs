//^
//^ HEAD
//^

//> HEAD -> FLAGS
#![no_std]
#![no_main]
#![allow(unused_variables)]
#![allow(static_mut_refs)]
#![allow(non_snake_case)]

//> HEAD -> SYSTEM CRATES
extern crate alloc;

//> HEAD -> CONTEXT
mod context {
    pub mod _infinity;
    pub mod _number;
    pub mod _undefined;
    pub mod _variable;
}

//> HEAD -> DATA
mod data {
    pub mod comment;
    pub mod debug;
    pub mod declaration;
    pub mod definition;
    pub mod equation;
    pub mod expression;
    pub mod factor;
    pub mod infinite;
    pub mod limit;
    pub mod nest;
    pub mod node;
    pub mod number;
    pub mod start;
    pub mod term;
    pub mod variable;
    pub mod vector;
}

//> HEAD -> LIB
mod lib {
    pub mod allocator;
    pub mod converter;
    pub mod formatting;
    pub mod runtime;
    pub mod rustc;
    pub mod stack;
    pub mod stdout;
}


//^
//^ PULLS
//^

//> PULLS -> CONTEXT
use context::_infinity::_Infinity;
use context::_number::_Number;
use context::_undefined::_Undefined;
use context::_variable::_Variable;

//> PULLS -> DATA
use data::comment::Comment;
use data::debug::Debug;
use data::declaration::Declaration;
use data::definition::Definition;
use data::equation::Equation;
use data::expression::Expression;
use data::factor::Factor;
use data::infinite::Infinite;
use data::limit::Limit;
use data::nest::Nest;
use data::node::Node;
use data::number::Number;
use data::start::Start;
use data::term::Term;
use data::variable::Variable;
use data::vector::Vector;

//> PULLS -> LIB
use lib::*;

//> PULLS -> ALLOC
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::alloc::Layout;

//> PULLS -> CORE
use core::sync::atomic::{AtomicUsize, Ordering};
use core::alloc::GlobalAlloc;
use core::panic::PanicInfo;


//^
//^ GLOBALS
//^

//> GLOBALS -> SETTINGS STRUCT
struct Settings {
    ir: &'static [u8],
    version: [usize; 3],
    detail: bool,
    lookup: bool,
    memsize: usize,
    precision: u8,
    width: u8
}

//> GLOBALS -> SETTINGS
static SETTINGS: Settings = Settings {
    ir: include_bytes!(env!("Mathsys")),
    version: [3, 5, 4],
    detail: true,
    lookup: true,
    memsize: 33554432,
    precision: if usize::BITS == 64 {3} else {2},
    width: 100
};

//> GLOBALS -> ALLOCATOR
#[global_allocator]
static ALLOCATOR: allocator::Allocator = allocator::Allocator::new();
static mut HEAP: [u8; SETTINGS.memsize] = [0; SETTINGS.memsize];


//^
//^ ENTRY
//^

//> ENTRY -> POINT
#[no_mangle]
pub extern "C" fn _start() -> ! {
    ALLOCATOR.init();
    stdout::login();
    ALLOCATOR.tempSpace(|| {
        stdout::debug(&format!(
            "Total heap size is {}B",
            formatting::scientific(SETTINGS.memsize).trim_start()
        ));
        stdout::debug(&format!(
            "Detail calls are {}",
            if SETTINGS.detail {"enabled"} else {"disabled"}
        ));
        stdout::debug(&format!(
            "Lookup calls are {}",
            if SETTINGS.lookup {"enabled"} else {"disabled"}
        ));
        stdout::debug(&format!(
            "Precision is set to {}",
            SETTINGS.precision
        ));
    });
    run();
    stdout::crash(0);
}

//> RUNTIME -> FUNCTION
fn run() -> () {
    stdout::space("Processing IR");
    let mut converter = converter::Converter::new();
    let memory = converter.run();
    let mut context = runtime::Context::new(memory.len(), memory);
    let output = context.quick();
}