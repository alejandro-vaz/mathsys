//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::syntax::Start;

//> HEAD -> SUPER
use super::LaTeX;


//^
//^ START
//^

//> START -> IMPLEMENTATION
impl<'valid> LaTeX for Start<'valid> {
    fn render(&self) -> String {return self.statements.iter().map(LaTeX::render).filter(|string| {
        !string.is_empty()
    }).collect::<Vec<String>>().join(r"\\ ")}
}