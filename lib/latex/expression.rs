//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::syntax::expression::Expression;

//> HEAD -> SUPER
use super::LaTeX;


//^
//^ EXPRESSION
//^

//> EXPRESSION -> IMPLEMENTATION
impl<'valid> LaTeX for Expression<'valid> {
    fn render(&self) -> String {return self.terms.iter().map(|(signs, level3)| format!(
        "{}{}", 
        signs.into_iter().map(|sign| if *sign {'+'} else {'-'}).collect::<String>(), 
        level3.render()
    )).collect()}
}