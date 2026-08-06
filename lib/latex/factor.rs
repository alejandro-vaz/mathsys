//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::syntax::factor::{
    Raised,
    Limit
};

//> HEAD -> SUPER
use super::LaTeX;


//^
//^ FACTOR
//^

//> FACTOR -> RAISED
impl<'valid> LaTeX for Raised<'valid> {
    fn render(&self) -> String {return format!(
        "{}{}", 
        self.value.render(), 
        self.exponent.as_ref().map(|exponent| format!(
            "^{{{}}}", 
            exponent.render()
        )).unwrap_or_default()
    )}
}

//> FACTOR -> LIMIT
impl<'valid> LaTeX for Limit<'valid> {
    fn render(&self) -> String {return format!(
        "\\lim_{{{}\\to {}{}}}{}{}",
        self.identifier.render(),
        self.expression.render(),
        match self.direction {
            None => '\0',
            Some(false) => '-',
            Some(true) => '+'
        },
        self.nest.render(),
        self.exponent.as_ref().map(|exponent| format!(
            "^{{{}}}", 
            exponent.render()
        )).unwrap_or_default()
    )}
}