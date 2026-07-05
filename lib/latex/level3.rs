//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::solver::level3::Term;

//> HEAD -> SUPER
use super::LaTeX;


//^
//^ 3ºLEVEL
//^

//> 3ºLEVEL -> TERM
impl<'valid> LaTeX for Term<'valid> {
    fn render(&self) -> String {
        return if self.denominator.is_empty() {format!(
            "{}",
            self.numerator.iter().map(LaTeX::render).collect::<String>()
        )} else {format!(
            "\\frac{{{}}}{{{}}}",
            self.numerator.iter().map(LaTeX::render).collect::<String>(),
            self.denominator.iter().map(LaTeX::render).collect::<String>()
        )}
    }
}