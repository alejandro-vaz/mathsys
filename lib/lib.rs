//^
//^ HEAD
//^

//> HEAD -> ATTRIBUTES
#![allow(incomplete_features)]
#![feature(phantom_variance_markers)]
#![feature(default_field_values)]
#![feature(generic_const_exprs)]

//> HEAD -> MODULES
mod extensor;
mod failure;
mod filter;
mod latex;
mod parser;
mod reducer;
mod solver;
mod tokenizer;

//> HEAD -> STD
use std::env::consts::{
    OS,
    ARCH
};

//> HEAD -> LIBUTILS
use libutils::active_reporting::Report;

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

//> HEAD -> REDUCER
use reducer::reduce;

//> HEAD -> CORE
use core::marker::PhantomCovariantLifetime;


//^
//^ INTERPRETER
//^

//> INTERPRETER -> RESOLVER
pub trait Resolver<'valid> {
    fn resolve(&'valid self, filename: &'valid str, report: Report<"Resolver">) -> &'valid str;
}

//> INTERPRETER -> STRUCT
pub struct Interpreter<'valid, Handler: Resolver<'valid>> {
    pub resolver: Handler,
    pub lifetime: PhantomCovariantLifetime<'valid>
} impl<'valid, Handler: Resolver<'valid>> Interpreter<'valid, Handler> {
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const OS: &'static str = OS;
    pub const ARCHITECTURE: &'static str = ARCH;
    pub fn latex(&'valid self, filename: &'valid str, mut report: Report<"Latex">) -> String {
        return solve(
            parse(
                filter(tokenize(
                    self.resolver.resolve(
                        filename,
                        report.to()
                    ),
                    report.to()
                )),
                extend(reduce()),
            ),
            None,
            filename,
            self,
            report.to()
        ).render();
    }
}