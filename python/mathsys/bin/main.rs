//
//  HEAD
//

// HEAD -> FLAGS
#![no_std]
#![no_main]
#![allow(unused_variables)]
#![allow(static_mut_refs)]

// HEAD -> SYSTEM CRATES
extern crate alloc;

// HEAD -> CUSTOM CRATES
pub mod exec;
pub mod object;
mod lib {
    pub mod allocator;
    pub mod memory;
    pub mod rustc;
    pub mod stdout;
    pub mod string;
}


//
//  PULLS
//

// PULLS -> LIB
use lib::*;

// PULLS -> ALLOC
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::alloc::Layout;

// PULLS -> CORE
use core::cell::UnsafeCell;
use core::alloc::GlobalAlloc;
use core::panic::PanicInfo;
use core::ptr::null_mut;

// PULLS -> OBJECT
use object::Node;
use object::{Factor, Placeholder};


//
//  GLOBALS
//

// GLOBALS -> SETTINGS STRUCT
struct Settings {
    ir: &'static str,
    version: &'static str,
    bcalls: bool,
    ncalls: bool,
    memsize: usize,
    precision: u8,
}

// GLOBALS -> SETTINGS
static SETTINGS: Settings = Settings {
    ir: include_str!(env!("IR")),
    version: "0.10.2",
    bcalls: true,
    ncalls: true,
    memsize: 4194304,
    precision: 2,
};

// GLOBALS -> ALLOCATOR
#[global_allocator]
static mut ALLOCATOR: allocator::Allocator = allocator::Allocator {
    start: 0,
    end: 0,
    next: UnsafeCell::new(0)
};
static mut HEAP: [u8; SETTINGS.memsize] = [0; SETTINGS.memsize];


//
//  ENTRY
//

// ENTRY -> POINT
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    allocator::init();
    stdout::login();
    runtime(string::split(SETTINGS.ir, "\n"));
    stdout::crash();
}


//
//  RUNTIME
//

// RUNTIME -> MEMORY
struct Memory {
    memory: Vec<Node>
}
impl Memory {
    fn new() -> Self {
        return Memory {
            memory: Vec::new()
        }
    }
    fn set(&mut self, index: usize, value: Node) -> () {
        while self.memory.len() <= index {
            self.memory.push(Node::Placeholder(Placeholder {}));
        }
        self.memory[index] = value;
    }
    fn get(&mut self, index: usize) -> Option<&Node> {
        return self.memory.get(index);
    }
}

// RUNTIME -> LOCATION
fn location(representation: &str) -> usize {
    return string::split(representation, "$")[1].parse().unwrap();
}

// RUNTIME -> FUNCTION
unsafe fn runtime(instructions: Vec<String>) -> () {
    stdout::space("Processing intermediate representation...");
    let mut memory = Memory::new();
    for instruction in &instructions {
        stdout::debug(instruction);
        let elements = string::split(&instruction, " ");
        match &elements[0] as &str {
            "factor" => memory.set(location(&elements[1]), Node::Factor(factor(&elements[1..]))),
            _ => {}
        }
    }
}

// RUNTIME -> FACTOR
unsafe fn factor(arguments: &[String]) -> Factor {
    let node = Factor {
        sign: ALLOCATOR.alloc(Layout::new::<u8>()),
        precision: ALLOCATOR.alloc(Layout::new::<u8>()),
        value: ALLOCATOR.alloc(Layout::new::<u8>())
    };
    *node.sign = 0x01;
    return node;
}