//^
//^ HEAD
//^

//> HEAD -> ATTRIBUTES
#![allow(incomplete_features)]
#![feature(default_field_values)]
#![feature(const_trait_impl)]
#![feature(new_range)]
#![feature(phantom_variance_markers)]
#![feature(generic_const_exprs)]

//> HEAD -> MODULES
mod failure;
mod latex;
mod parser;
mod runtime;
mod syntax;

//> HEAD -> PARSER
use parser::parse;

//> HEAD -> LATEX
use latex::LaTeX;

//> HEAD -> FAILURE
pub use failure::Failure;

//> HEAD -> RUNTIME
pub use runtime::Runtime;

//> HEAD -> CORE
use core::marker::PhantomCovariantLifetime;


//^
//^ INTERPRETER
//^

//> INTERPRETER -> STRUCT
pub struct Interpreter<'valid, Implementation: Runtime<'valid>> {
    runtime: Implementation,
    _lifetime: PhantomCovariantLifetime<'valid>
} 

//> INTERPRETER -> IMPLEMENTATION
impl<'valid, Implementation: Runtime<'valid>> Interpreter<'valid, Implementation> {
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub fn latex(
        &'valid self,
        filename: &'valid str
    ) -> String {return parse::<Implementation>(
        self.runtime.resolve(filename),
    ).render()}
}

//> INTERPRETER -> FROM IMPLEMENTATION
impl<
    'valid, 
    Implementation: Runtime<'valid>
> From<Implementation> for Interpreter<'valid, Implementation> {
    fn from(value: Implementation) -> Self {return Self {
        runtime: value,
        _lifetime: PhantomCovariantLifetime::new()
    }}
}