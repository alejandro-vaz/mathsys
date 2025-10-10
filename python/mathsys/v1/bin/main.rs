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
mod data {}

// HEAD -> LIB
mod lib {
    pub mod allocator;
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

// PULLS -> LIB
use lib::*;

// PULLS -> DATA

// PULLS -> ALLOC
use alloc::vec::Vec;
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
    version: [1, 4, 124],
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
            "Total heap size is {} bytes",
            SETTINGS.memsize
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

// RUNTIME -> FUNCTION
fn runtime() -> () {
    stdout::space("Here is as far as the current version goes, not IR yet");
}