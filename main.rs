//^
//^ HEAD
//^

//> HEAD -> FLAGS
#![allow(unused_variables)]
#![allow(nonstandard_style)]
#![feature(if_let_guard)]

//> HEAD -> LOCAL
use mathsys::{
    prelude::ExitCode,
    TRANSFORMERS,
    settings::{
        Settings,
        noise::Noise
    },
    Data
};

//> TARGETS -> WRAPPER
pub fn main() -> ExitCode {
    let settings = match Settings::cli() {
        Ok(settings) => settings,
        Err(issue) => return ExitCode::from(issue.consume())
    };
    for result in TRANSFORMERS.run(&settings) {
        match result.data {
            Err(issue) => return ExitCode::from(issue.consume()),
            Ok(Data::Version {mathsys, architecture, os, rust}) => if settings.noise.verbose() {println!("Running Mathsys v{mathsys} on {architecture}.{os}~{rust}.")} else {println!("Running Mathsys v{mathsys}.")},
            Ok(Data::Tokens {length, tokens, maximum, percentage}) => match settings.noise {
                Noise::Debug => println!("{length} / {maximum} ({percentage}%)\n{tokens:#?}"),
                Noise::Verbose => println!("{length} / {maximum} ({percentage}%)"),
                Noise::Normal | Noise::Quiet => println!("{length} / {maximum}"),
                Noise::Zero => println!("{length}")
            },
            Ok(Data::Check) => if !settings.noise.quiet() {println!("No issues found.")},
            Ok(Data::Ast {start}) => println!("{start:#?}"),
            Ok(Data::Latex {representation}) => println!("{representation}")
        }
        if settings.noise.debug() {println!("Executed in {:#?}.", result.time)};
    };
    return ExitCode::SUCCESS;
}