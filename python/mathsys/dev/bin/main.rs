//^
//^ HEAD
//^

//> HEAD -> FLAGS
#![no_std]
#![no_main]
#![allow(unused_variables)]
#![allow(static_mut_refs)]
#![allow(non_snake_case)]
#![feature(const_cmp)]
#![feature(const_trait_impl)]

//> HEAD -> SYSTEM CRATES
extern crate alloc;

//> HEAD -> CONTEXT
mod context {
    pub mod _infinity;
    pub mod _nexists;
    pub mod _number;
    pub mod _undefined;
    pub mod _variable;
}

//> HEAD -> DATA
mod data {
    pub mod comment;
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
use context::_nexists::_Nexists;
use context::_number::_Number;
use context::_undefined::_Undefined;
use context::_variable::_Variable;

//> PULLS -> DATA
use data::comment::Comment;
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
use core::sync::atomic::{Ordering, AtomicBool};
use core::alloc::GlobalAlloc;
use core::panic::PanicInfo;
use core::cmp::max;


//^
//^ GLOBALS
//^

//> GLOBALS -> SETTINGS STRUCT
struct Settings {
    ir: &'static [u8],
    version: [&'static str; 3],
    memsize: usize,
    block: usize,
    precision: u8,
    width: u8
}

//> GLOBALS -> SETTINGS
static SETTINGS: Settings = Settings {
    ir: include_bytes!(env!("MathsysSource")),
    version: [
        env!("MathsysMajor"), 
        env!("MathsysMinor"), 
        env!("MathsysPatch")
    ],
    memsize: match env!("MathsysMemory") {
        "page" => 2usize.pow(12),
        "basic" => 2usize.pow(16),
        "maximum" => 2usize.pow(22),
        other => 2usize.pow(16)
    },
    block: match env!("MathsysOptimization") {
        "very low" => 2usize.pow(7),
        "low" => 2usize.pow(6),
        "default" => 2usize.pow(5),
        "high" => 2usize.pow(4),
        "very high" => 2usize.pow(3),
        other => 2usize.pow(5)
    },
    precision: match env!("MathsysPrecision") {
        "reduced" => 2,
        "standard" => if usize::BITS == 64 {3} else {2},
        other => if usize::BITS == 64 {3} else {2}
    },
    width: 100
};

//> GLOBALS -> ALLOCATOR
#[repr(align(128))]
struct AlignedHeap {data: [u8; SETTINGS.memsize]}
static mut HEAP: AlignedHeap = AlignedHeap {data: [0; SETTINGS.memsize]};
#[global_allocator]
static ALLOCATOR: allocator::Allocator = allocator::Allocator::new();


//^
//^ ENTRY
//^

//> ENTRY -> POINT
#[no_mangle]
pub extern "C" fn _start() -> ! {
    ALLOCATOR.reset();
    stdout::login();
    stdout::debug(&format!(
        "Total heap size is {}B",
        formatting::scientific(SETTINGS.memsize).trim_start()
    ));
    stdout::debug(&format!(
        "There are {} memory blocks, each of {}B",
        formatting::scientific(SETTINGS.memsize / SETTINGS.block).trim_start(),
        formatting::scientific(SETTINGS.block).trim_start()
    ));
    stdout::debug(&format!(
        "Precision is set to {}",
        SETTINGS.precision
    ));
    run();
    stdout::crash(stdout::Code::Success);
}

//> RUNTIME -> FUNCTION
fn run() -> () {
    stdout::space("Processing IR");
    let mut converter = converter::Converter::new();
    let memory = converter.run();
    let mut context = runtime::Context::new(memory.len(), memory);
    let output = context.quick();
}