//^
//^ HEAD
//^

//> HEAD -> ATTRIBUTES
#![allow(incomplete_features)]
#![feature(guard_patterns)]
#![feature(phantom_variance_markers)]
#![feature(generic_const_exprs)]

//> HEAD -> MODULES
mod extensor;
mod failure;
mod filter;
mod latex;
mod parser;
mod solver;
mod tokenizer;

//> HEAD -> STD
use std::env::consts::{
    OS,
    ARCH
};

//> HEAD -> LIBUTILS
use libutils::report::{
    Report,
    Name
};

//> HEAD -> TOKENIZER
use tokenizer::tokenize;

//> HEAD -> FILTER
use filter::filter;

//> HEAD -> PARSER
use parser::parse;

//> HEAD -> EXTENSOR
use extensor::extend;

//> HEAD -> SOLVER
use solver::solve;

//> HEAD -> LATEX
use latex::LaTeX;

//> HEAD -> FAILURE
pub use failure::Failure;

//> HEAD -> CORE
use core::marker::PhantomCovariantLifetime;


//^
//^ INTERPRETER
//^

//> INTERPRETER -> RESOLVER
pub trait Resolver<'valid> {
    fn resolve(&'valid self, filename: &'valid str, report: Report<Name<'_, "Resolver">>) -> Option<&'valid str>;
}

//> INTERPRETER -> STRUCT
pub struct Interpreter<'valid, Handler: Resolver<'valid>> {
    pub resolver: Handler,
    pub lifetime: PhantomCovariantLifetime<'valid>
} impl<'valid, Handler: Resolver<'valid>> Interpreter<'valid, Handler> {
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const OS: &'static str = OS;
    pub const ARCHITECTURE: &'static str = ARCH;
    pub fn latex(&'valid self, filename: &'valid str, mut report: Report<Name<"Latex">>) -> Option<String> {
        return Some(solve(
            parse(
                filter(
                    tokenize(
                        self.resolver.resolve(filename, report.to())?, 
                        report.to()
                    )?,
                    report.to()
                )?,
                extend(report.to())?,
                report.to()
            )?,
            None,
            filename,
            self,
            report.to()
        )?.render());
    }
}