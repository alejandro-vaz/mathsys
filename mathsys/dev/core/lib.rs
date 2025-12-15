//^
//^ HEAD
//^

//> HEAD -> FLAGS
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![feature(const_cmp)]
#![feature(const_trait_impl)]

//> HEAD -> CONTEXT
mod context {
    pub mod infinite;
    pub mod nexists;
    pub mod number;
    pub mod tensor;
    pub mod undefined;
    pub mod variable;
}

//> HEAD -> DATA
mod data {
    pub mod _annotation;
    pub mod _comment;
    pub mod _declaration;
    pub mod _definition;
    pub mod _equation;
    pub mod _expression;
    pub mod _factor;
    pub mod _infinite;
    pub mod _limit;
    pub mod _nest;
    pub mod _node;
    pub mod _number;
    pub mod _start;
    pub mod _tensor;
    pub mod _term;
    pub mod _use;
    pub mod _variable;
}

//> HEAD -> LIB
mod lib {
    pub mod class;
    pub mod group;
    pub mod object;
    pub mod reparser;
    pub mod runtime;
    pub mod stack;
    pub mod stdout;
    pub mod tip;
    pub mod value;
}


//^
//^ PULLS
//^

//> PULLS -> CONTEXT
use context::infinite::Infinite;
use context::nexists::Nexists;
use context::number::Number;
use context::tensor::Tensor;
use context::undefined::Undefined;
use context::variable::Variable;

//> PULLS -> DATA
use data::_annotation::_Annotation;
use data::_comment::_Comment;
use data::_declaration::_Declaration;
use data::_definition::_Definition;
use data::_equation::_Equation;
use data::_expression::_Expression;
use data::_factor::_Factor;
use data::_infinite::_Infinite;
use data::_limit::_Limit;
use data::_nest::_Nest;
use data::_node::_Node;
use data::_number::_Number;
use data::_start::_Start;
use data::_tensor::_Tensor;
use data::_term::_Term;
use data::_use::_Use;
use data::_variable::_Variable;

//> PULLS -> LIB
use lib::*;

//> PULLS -> STD
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::cmp::max;


//^
//^ GLOBALS
//^

//> GLOBALS -> SETTINGS STRUCT
pub struct Settings {
    pub ir: &'static [u8],
    pub version: [&'static str; 3]
}


//^
//^ ENTRY
//^

//> RUNTIME -> FUNCTION
pub fn run(settings: Settings) -> () {
    stdout::login(&settings);
    let mut reparser = reparser::Reparser::new();
    let memory = reparser.run(&settings);
    let mut runtime = runtime::Runtime::new();
    runtime.start(memory);
    stdout::crash(stdout::Code::Success);
}