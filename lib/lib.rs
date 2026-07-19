//^
//^ HEAD
//^

//> HEAD -> ATTRIBUTES
#![allow(incomplete_features)]
#![feature(phantom_variance_markers)]
#![feature(alloc_slice_into_array)]
#![feature(default_field_values)]
#![feature(str_from_raw_parts)]
#![feature(generic_const_exprs)]

//> HEAD -> MODULES
mod failure;
mod filter;
mod grammar;
mod latex;
mod parser;
mod solver;
mod tokenizer;

//> HEAD -> LIBUTILS
use libutils::active_reporting::Report;

//> HEAD -> TOKENIZER
use tokenizer::tokenize;

//> HEAD -> FILTER
use filter::filter;

//> HEAD -> PARSER
use parser::parse;

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
    fn resolve(
        &'valid self, 
        filename: &'valid str, 
        report: Report<"Resolver">
    ) -> &'valid str;
}

//> INTERPRETER -> STRUCT
pub struct Interpreter<'valid, Handler: Resolver<'valid>> {
    pub resolver: Handler,
    pub lifetime: PhantomCovariantLifetime<'valid>,
    pub warning: fn(Failure, &[&'static str]) -> (),
    pub error: fn(Failure, &[&'static str]) -> (),
    pub critical: fn(Failure, &[&'static str]) -> !
} impl<'valid, Handler: Resolver<'valid>> Interpreter<'valid, Handler> {
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub fn latex(&'valid self, filename: &'valid str, mut report: Report<"Latex">) -> String {
        return solve(
            parse(filter(tokenize(
                self.resolver.resolve(
                    filename,
                    report.to()
                ),
                report.to(),
                self
            ))),
            None,
            filename,
            self,
            report.to()
        ).render();
    }
}