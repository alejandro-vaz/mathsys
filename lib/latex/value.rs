//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::syntax::value::{
    Infinite,
    Identifier,
    Nest,
    Vector,
    Number,
    Absolute,
    Undefined,
    Call
};

//> HEAD -> SUPER
use super::LaTeX;


//^
//^ VALUE
//^

//> VALUE -> INFINITE
impl LaTeX for Infinite {
    fn render(&self) -> String {return String::from(r"\infty ")}
}

//> VALUE -> IDENTIFIER
impl<'valid> LaTeX for Identifier<'valid> {
    fn render(&self) -> String {return self.name.to_string()}
}

//> VALUE -> NEST
impl<'valid> LaTeX for Nest<'valid> {
    fn render(&self) -> String {return format!(
        r"\left( {}\right) ", 
        self.inside.as_ref().map(LaTeX::render).unwrap_or_default()
    )}
}

//> VALUE -> VECTOR
impl<'valid> LaTeX for Vector<'valid> {
    fn render(&self) -> String {return format!(
        r"\begin{{bmatrix}}{}\end{{bmatrix}}", 
        match self.expressions.len() {
            0 => String::from(r"\; "),
            _ => self.expressions.iter().map(LaTeX::render).collect::<Vec<String>>().join(r"\\ ")
        }
    )}
}

//> VALUE -> NUMBER
impl<'valid> LaTeX for Number<'valid> {
    fn render(&self) -> String {return self.number.to_string()}
}

//> VALUE -> ABSOLUTE
impl<'valid> LaTeX for Absolute<'valid> {
    fn render(&self) -> String {
        return format!(r"\left| {}\right| ", self.expression.render());
    }
}

//> VALUE -> UNDEFINED
impl LaTeX for Undefined {
    fn render(&self) -> String {return String::from(r"\left. ?\right. ")}
}

//> VALUE -> CALL
impl<'valid> LaTeX for Call<'valid> {
    fn render(&self) -> String {return format!(
        r"{}\left( {}\right) ", 
        self.identifier.render(), 
        self.with.iter().map(LaTeX::render).collect::<Vec<String>>().join(",")
    )}
}