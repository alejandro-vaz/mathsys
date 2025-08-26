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
mod data {}

// HEAD -> LIB
mod lib {
    pub mod allocator;
    pub mod memory;
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
    bcalls: bool,
    ncalls: bool,
    memsize: usize,
    precision: u8,
    width: u8
}

// GLOBALS -> SETTINGS
static SETTINGS: Settings = Settings {
    ir: include_bytes!(env!("Mathsys")),
    version: [1, 2, 7],
    bcalls: true,
    ncalls: true,
    memsize: 33554432,
    precision: if usize::BITS == 64 {3} else {2},
    width: 80
};

// GLOBALS -> ALLOCATOR
#[global_allocator]
static mut ALLOCATOR: allocator::Allocator = allocator::Allocator {
    start: 0,
    end: 0,
    next: AtomicUsize::new(0)
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
    stdout::trace(&format!(
        "Available memory size is {} bytes",
        SETTINGS.memsize
    ));
    stdout::debug(&format!(
        "B calls are {}",
        if SETTINGS.bcalls {"enabled"} else {"disabled"}
    ));
    stdout::debug(&format!(
        "N calls are {}",
        if SETTINGS.ncalls {"enabled"} else {"disabled"}
    ));
    stdout::debug(&format!(
        "Precision is set to {}",
        SETTINGS.precision
    ));
    runtime();
    stdout::crash(0);
}


//
//  RUNTIME
//


// RUNTIME -> FUNCTION
unsafe fn runtime() -> () {
    stdout::space("Here is as far as the current version goes, not IR yet");
}