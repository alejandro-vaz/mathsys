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

// HEAD -> DATA
mod data {
    pub mod number;
    pub mod placeholder;
}

// HEAD -> LIB
mod lib {
    pub mod allocator;
    pub mod memory;
    pub mod rustc;
    pub mod stdout;
    pub mod string;
}

// HEAD -> STACK
pub mod stack {
    pub mod number;
    pub mod system;
}


//
//  PULLS
//

// PULLS -> LIB
use lib::*;

// PULLS -> DATA
use data::number::Number;
use data::placeholder::Placeholder;

// PULLS -> ALLOC
use alloc::vec::Vec;
use alloc::string::String;
use alloc::alloc::Layout;

// PULLS -> CORE
use core::cell::UnsafeCell;
use core::alloc::GlobalAlloc;
use core::panic::PanicInfo;
use core::ptr::null_mut;
use core::str::from_utf8;


//
//  GLOBALS
//

// GLOBALS -> SETTINGS STRUCT
struct Settings {
    ir: &'static [u8],
    version: &'static str,
    bcalls: bool,
    ncalls: bool,
    memsize: usize,
    precision: u8,
}

// GLOBALS -> SETTINGS
static SETTINGS: Settings = Settings {
    ir: include_bytes!(env!("IR")),
    version: "0.10.2",
    bcalls: true,
    ncalls: true,
    memsize: 4194304,
    precision: if usize::BITS == 32 {2} else {3}
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
    stdout::trace(&string::join(&[
        "Allocated memory size is ",
        &alloc::format!("{}", SETTINGS.memsize),
        " bytes."
    ]));
    stdout::trace(&string::join(&[
        "Intermediate Representation loaded with a length of ",
        &alloc::format!("{}", SETTINGS.ir.len()),
        "."
    ]));
    stdout::debug(&string::join(&[
        "B calls are ",
        if SETTINGS.bcalls {"enabled."} else {"disabled."}
    ]));
    stdout::debug(&string::join(&[
        "N calls are ",
        if SETTINGS.ncalls {"enabled."} else {"disabled."}
    ]));
    stdout::debug(&string::join(&[
        "Calculation precision is set to ",
        &alloc::format!("{}", SETTINGS.precision),
        "."
    ]));
    runtime();
    stdout::crash();
}


//
//  RUNTIME
//

// RUNTIME -> NODE
pub enum Node {
    Number(Number),
    Placeholder(Placeholder)
}

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
    unsafe fn set(&mut self, index: usize, value: Node) -> () {
        while self.memory.len() < index {
            self.memory.push(Node::Placeholder(Placeholder::new()));
        }
        if self.memory.len() == index {
            self.memory.push(value);
        } else {
            self.memory[index] = value;
        }
    }
    fn get(&mut self, index: usize) -> Option<&Node> {
        return self.memory.get(index);
    }
}

// RUNTIME -> FUNCTION
unsafe fn runtime() -> () {
    stdout::space("Processing intermediate representation...");
    let mut memory = Memory::new();
    stdout::trace("Program memory initialized.");
    let mut index = 0;
    while index < SETTINGS.ir.len() - 1 {
        match SETTINGS.ir[index] {
            0x0A => {
                index += 1;
                let adr = SETTINGS.ir[index];
                index += 1;
                let signs = SETTINGS.ir[index];
                index += 1;
                let length = SETTINGS.ir[index];
                index += 1;
                let value = &SETTINGS.ir[index..index+(length as usize)];
                index += length as usize;
                let exponent = SETTINGS.ir[index];
                index += if exponent == 0x01 {1} else {0};
                let exponent_adr: Option<u8> = if exponent == 0x01 {Some(SETTINGS.ir[index])} else {None};
                memory.set(adr as usize, Node::Number(Number::new()));
            },
            _ => {}
        };
        index += 1;
    }
}