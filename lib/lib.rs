//^
//^ HEAD
//^

//> HEAD -> ATTRIBUTES
#![allow(incomplete_features)]
#![feature(default_field_values)]
#![feature(const_default)]
#![feature(const_trait_impl)]
#![feature(nonzero_ops)]
#![feature(new_range)]
#![feature(phantom_variance_markers)]
#![feature(generic_const_exprs)]

//> HEAD -> MODULES
mod failure;
mod filter;
mod latex;
mod parser;
mod pruner;
mod runtime;
mod solver;
mod syntax;
mod tokenizer;

//> HEAD -> TOKENIZER
use tokenizer::tokenize;

//> HEAD -> FILTER
use filter::filter;

//> HEAD -> PARSER
use parser::parse;

//> HEAD -> SOLVER
//use solver::{
//    solve,
//    context::Context
//};

//> HEAD -> LATEX
use latex::LaTeX;

//> HEAD -> FAILURE
pub use failure::Failure;

//> HEAD -> RUNTIME
pub use runtime::Runtime;

//> HEAD -> CORE
use core::marker::PhantomCovariantLifetime;

//> HEAD -> PRUNER
use pruner::prune;


//^
//^ INTERPRETER
//^

//> INTERPRETER -> STRUCT
pub struct Interpreter<'valid, Implementation: Runtime<'valid>> {
    pub runtime: Implementation,
    pub lifetime: PhantomCovariantLifetime<'valid>
} 

//> INTERPRETER -> IMPLEMENTATION
impl<'valid, Implementation: Runtime<'valid>> Interpreter<'valid, Implementation> {
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub fn latex(
        &'valid self,
        filename: &'valid str
    ) -> String {prune(parse(&filter(tokenize(
        self.runtime.resolve(filename),
        filename,
        &self.runtime
    )))); String::new()}
}