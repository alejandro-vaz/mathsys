//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod level1;
pub mod level2;
pub mod level3;
pub mod level4;
pub mod level5;
pub mod start;

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