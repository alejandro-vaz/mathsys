//
//  HEAD
//

// HEAD -> FLAGS
#![no_std]
#![no_main]
#![allow(unused_variables)]
#![allow(static_mut_refs)]
#![allow(non_snake_case)]

// HEAD -> SYSTEM CRATES
extern crate alloc;

// HEAD -> DATA
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

// HEAD -> LIB
mod lib {
    pub mod allocator;
    pub mod converter;
    pub mod memory;
    pub mod number;
    pub mod rustc;
    pub mod stdout;
}

// HEAD -> STACK
pub mod stack {
    pub mod system;
}


//
//  PULLS
//

// PULLS -> LIB AND DATA
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
use lib::*;

// PULLS -> DATA

// PULLS -> ALLOC
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::alloc::Layout;

// PULLS -> CORE
use core::sync::atomic::{AtomicUsize, Ordering};
use core::alloc::GlobalAlloc;
use core::panic::PanicInfo;


//
//  GLOBALS
//

// GLOBALS -> SETTINGS STRUCT
struct Settings {
    ir: &'static [u8],
    version: [usize; 3],
    detail: bool,
    lookup: bool,
    memsize: usize,
    precision: u8,
    width: u8
}

// GLOBALS -> SETTINGS
static SETTINGS: Settings = Settings {
    ir: include_bytes!(env!("Mathsys")),
    version: [2, 1, 5],
    detail: true,
    lookup: true,
    memsize: 33554432,
    precision: if usize::BITS == 64 {3} else {2},
    width: 80
};

// GLOBALS -> ALLOCATOR
#[global_allocator]
static ALLOCATOR: allocator::Allocator = allocator::Allocator::new();
static mut HEAP: [u8; SETTINGS.memsize] = [0; SETTINGS.memsize];


//
//  ENTRY
//

// ENTRY -> POINT
#[no_mangle]
pub extern "C" fn _start() -> ! {
    ALLOCATOR.init();
    stdout::login();
    ALLOCATOR.tempSpace(|| {
        stdout::trace(&format!(
            "Total heap size is {}B",
            number::scientific(SETTINGS.memsize).trim_start()
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
    runtime();
    stdout::crash(0);
}


//
//  RUNTIME
//

// RUNTIME -> OBJECT
pub trait Object {}

// RUNTIME -> FUNCTION
fn runtime() -> () {
    stdout::space("Processing IR");
    let mut converter = converter::Converter::new();
    converter.run();
}