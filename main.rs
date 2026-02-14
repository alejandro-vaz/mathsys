//^
//^ HEAD
//^

//> HEAD -> FLAGS
#![allow(unused_variables)]
#![allow(nonstandard_style)]
#![feature(try_trait_v2)]
#![feature(default_field_values)]

//> HEAD -> PRELUDE
mod prelude;
use prelude::{
    wrapperDev
};

//> HEAD -> ENTRY
mod entry;
mod dev;
use dev::Issue;


//^
//^ MAIN
//^

//> MAIN -> VERSION
pub static VERSION: usize = 7;

//> MAIN -> EXECUTION
fn main() -> Result<(), Issue> {
    wrapperDev()
}