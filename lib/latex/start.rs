//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::solver::start::Start;

//> HEAD -> SUPER
use super::LaTeX;


//^
//^ START
//^

//> START -> LATEX
impl<'valid> LaTeX for Start<'valid> {
    fn render(&self) -> String {return self.stream.iter().map(LaTeX::render).filter(|string| {
        !string.is_empty()
    }).collect::<Vec<String>>().join("\n")}
}