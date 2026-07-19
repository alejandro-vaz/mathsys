//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::solver::level1::{
    Definition,
    Function,
    Node,
    Equation,
    Use
};

//> HEAD -> SUPER
use super::LaTeX;


//^ 
//^ 1ºLEVEL
//^ 

//> 1ºLEVEL -> DEFINITION
impl<'valid> LaTeX for Definition<'valid> {
    fn render(&self) -> String {
        return format!("{}:={}", self.variable.render(), self.value.render());
    }
}

//> 1ºLEVEL -> FUNCTION
impl<'valid> LaTeX for Function<'valid> {
    fn render(&self) -> String {return format!(
        r"{}\left( {}\right) :={}", 
        self.variable.render(),
        self.arguments.iter().map(LaTeX::render).collect::<Vec<String>>().join(","),
        self.expression.render()
    )}
}

//> 1ºLEVEL -> NODE
impl<'valid> LaTeX for Node<'valid> {
    fn render(&self) -> String {return self.value.render()}
}

//> 1ºLEVEL -> EQUATION
impl<'valid> LaTeX for Equation<'valid> {
    fn render(&self) -> String {return format!(
        "{}={}", 
        self.left.render(), 
        self.right.render()
    )}
}

//> 1ºLEVEL -> USE
impl<'valid> LaTeX for Use<'valid> {
    fn render(&self) -> String {return String::new()}
}