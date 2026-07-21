//^
//^ HEAD
//^

//> HEAD -> ATTRIBUTES
#![allow(incomplete_features)]
#![feature(alloc_slice_into_array)]
#![feature(default_field_values)]
#![feature(nonzero_ops)]
#![feature(generic_const_exprs)]

//> HEAD -> MODULES
mod failure;
mod filter;
mod grammar;
mod latex;
mod parser;
mod solver;
mod syntax;
mod tokenizer;

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::Report,
    systemio::SystemIO
};

//> HEAD -> TOKENIZER
use tokenizer::tokenize;

//> HEAD -> FILTER
use filter::filter;

//> HEAD -> PARSER
use parser::parse;

//> HEAD -> SOLVER
use solver::{
    solve,
    context::Context
};

//> HEAD -> LATEX
use latex::LaTeX;

//> HEAD -> FAILURE
pub use failure::Failure;


//^
//^ INTERPRETER
//^

//> INTERPRETER -> STRUCT
pub struct Interpreter<'valid> {
    pub resolver: fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
    pub systemio: SystemIO<Failure<'valid>>
} 

//> INTERPRETER -> IMPLEMENTATION
impl<'valid> Interpreter<'valid> {
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub fn latex(
        &'valid self, 
        filename: &'valid str, 
        mut report: Report<"Latex">
    ) -> String {return solve(
        parse(filter(tokenize(
            (self.resolver)(
                filename,
                report.to()
            ),
            filename,
            &self.systemio,
            report.to()
        ))),
        &mut Context::default(),
        filename,
        &self.systemio,
        &self.resolver,
        report.to()
    ).render()}
}