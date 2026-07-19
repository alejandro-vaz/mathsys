//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::solver::level4::{
    Factor,
    Limit
};

//> HEAD -> SUPER
use super::LaTeX;


//^
//^ 4ºLEVEL
//^

//> 4ºLEVEL -> FACTOR
impl<'valid> LaTeX for Factor<'valid> {
    fn render(&self) -> String {return format!(
        "{}{}", 
        self.value.render(), 
        self.exponent.as_ref().map(|exponent| format!(
            "^{{{}}}", 
            exponent.render()
        )).unwrap_or_default()
    )}
}

//> 4ºLEVEL -> LIMIT
impl<'valid> LaTeX for Limit<'valid> {
    fn render(&self) -> String {return format!(
        "\\lim_{{{}\to {}{}}}({}){}",
        self.variable.render(),
        self.approach.render(),
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