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
    getArguments, File, Flag, Alias, Target, Argument, wrapperDev
};

//> HEAD -> ENTRY
mod entry;
mod dev;


//^
//^ MAIN
//^

//> MAIN -> VERSION
pub static VERSION: usize = 6;

//> MAIN -> EXECUTION
fn main() -> () {
    let mut arguments = Vec::new();
    for argument in getArguments().skip(1) {match &argument {
        file if file.split(".").last().unwrap().starts_with("ms") => arguments.push(Argument::File(File {
            name: argument.into()
        })),
        flag if flag.starts_with("--") => arguments.push(Argument::Flag(Flag {
            value: argument.chars().skip(2).collect()
        })),
        alias if alias.starts_with("-") => arguments.push(Argument::Alias(Alias {
            letters: argument.chars().collect()
        })),
        target => arguments.push(Argument::Target(Target {
            name: argument
        })),
    }};
    call(arguments.iter().find_map(|argument| {
        if let Argument::File(file) = argument {Some(file.version())} else {None}
    }).unwrap_or(VERSION), arguments);
}

//> MAIN -> CALL
fn call(version: usize, arguments: Vec<Argument>) -> () {if let Err(issue) = match version {
    1 => return,
    2 => return,
    3 => return,
    4 => return,
    5 => return,
    6 => return,
    7 => wrapperDev(arguments),
    other => return
} {issue.consume()}}