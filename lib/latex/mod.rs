//^
//^ HEAD
//^

//> HEAD -> MODULES
mod level1;
mod level2;
mod level3;
mod level4;
mod level5;
mod start;

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