//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod expression;
pub mod factor;
pub mod start;
pub mod statement;
pub mod term;
pub mod value;

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;


//^
//^ LATEX
//^

//> LATEX -> TRAIT
#[enum_dispatch]
pub trait LaTeX {
    fn render(&self) -> String;
}