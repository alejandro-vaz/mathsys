//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::syntax::statement::{
    Definition,
    Function,
    Node,
    Equation,
    Use
};

//> HEAD -> SUPER
use super::LaTeX;


//^ 
//^ STATEMENT
//^ 

//> STATEMENT -> DEFINITION
impl<'valid> LaTeX for Definition<'valid> {
    fn render(&self) -> String {
        return format!("{}:={}", self.identifier.render(), self.expression.render());
    }
}

//> STATEMENT -> FUNCTION
impl<'valid> LaTeX for Function<'valid> {
    fn render(&self) -> String {return format!(
        r"{}\left( {}\right) :={}", 
        self.identifier.render(),
        self.arguments.iter().map(LaTeX::render).collect::<Vec<String>>().join(","),
        self.expression.render()
    )}
}

//> STATEMENT -> NODE
impl<'valid> LaTeX for Node<'valid> {
    fn render(&self) -> String {return self.expression.render()}
}

//> STATEMENT -> EQUATION
impl<'valid> LaTeX for Equation<'valid> {
    fn render(&self) -> String {
        return self.expressions.each_ref().map(LaTeX::render).join("=");
    }
}

//> STATEMENT -> USE
impl<'valid> LaTeX for Use<'valid> {
    fn render(&self) -> String {return String::new()}
}