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
#![feature(allocator_api)]

//> HEAD -> SYSTEM CRATES
extern crate alloc;

//> HEAD -> CONTEXT
mod context {
    pub mod infinite;
    pub mod nexists;
    pub mod number;
    pub mod tensor;
    pub mod undefined;
    pub mod variable;
}

//> HEAD -> DATA
mod data {
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
    pub mod _number;
    pub mod _start;
    pub mod _tensor;
    pub mod _term;
    pub mod _use;
    pub mod _variable;
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
use context::infinite::Infinite;
use context::nexists::Nexists;
use context::number::Number;
use context::tensor::Tensor;
use context::undefined::Undefined;
use context::variable::Variable;

//> PULLS -> DATA
use data::_annotation::_Annotation;
use data::_comment::_Comment;
use data::_declaration::_Declaration;
use data::_definition::_Definition;
use data::_equation::_Equation;
use data::_expression::_Expression;
use data::_factor::_Factor;
use data::_infinite::_Infinite;
use data::_limit::_Limit;
use data::_nest::_Nest;
use data::_node::_Node;
use data::_number::_Number;
use data::_start::_Start;
use data::_tensor::_Tensor;
use data::_term::_Term;
use data::_use::_Use;
use data::_variable::_Variable;

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
    precision: u8
}

//> GLOBALS -> SETTINGS
static SETTINGS: Settings = Settings {
    ir: include_bytes!(env!("MathsysSource")),
    version: [
        env!("MathsysMajor"), 
        env!("MathsysMinor"), 
        env!("MathsysPatch")
    ],
    memsize: 4096,
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
    }
};


//^
//^ ENTRY
//^

//> ENTRY -> ALLOCATOR
#[global_allocator]
static ALLOCATOR: allocator::Allocator = allocator::Allocator::new();

//> ENTRY -> POINT
#[no_mangle]
pub extern "C" fn _start() -> ! {
    stdout::login();
    stdout::debug(&format!(
        "Total heap size is {}B",
        formatting::scientific(SETTINGS.memsize).trim_end()
    ));
    stdout::debug(&format!(
        "There are {} memory blocks, each of {}B",
        formatting::scientific(SETTINGS.memsize / SETTINGS.block).trim_end(),
        formatting::scientific(SETTINGS.block).trim_end()
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
    let mut converter = converter::Converter::new();
    let memory = converter.run();
    let mut context = runtime::Context::new(memory);
    let output = context.quick();
}