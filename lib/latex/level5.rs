//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::solver::level5::{
    Infinite,
    Variable,
    Nest,
    Vector,
    Whole,
    Absolute,
    Undefined,
    Rational,
    Call
};

//> HEAD -> SUPER
use super::LaTeX;


//^
//^ 5ºLEVEL
//^

//> 5ºLEVEL -> INFINITE
impl LaTeX for Infinite {
    fn render(&self) -> String {return String::from(r"\infty ")}
}

//> 5ºLEVEL -> VARIABLE
impl<'valid> LaTeX for Variable<'valid> {
    fn render(&self) -> String {return self.name.to_string()}
}

//> 5ºLEVEL -> NEST
impl<'valid> LaTeX for Nest<'valid> {
    fn render(&self) -> String {return format!(
        r"\left( {}\right) ", 
        self.value.as_ref().map(LaTeX::render).unwrap_or_default()
    )}
}

//> 5ºLEVEL -> VECTOR
impl<'valid> LaTeX for Vector<'valid> {
    fn render(&self) -> String {
        return format!(r"\begin{{bmatrix}}{}\end{{bmatrix}}", match self.values.len() {
            0 => String::from(r"\; "),
            _ => self.values.iter().map(LaTeX::render).collect::<Vec<String>>().join(r"\\ ")
        })
    }
}

//> 5ºLEVEL -> WHOLE
impl<'valid> LaTeX for Whole<'valid> {
    fn render(&self) -> String {return self.number.to_string()}
}

//> 5ºLEVEL -> ABSOLUTE
impl<'valid> LaTeX for Absolute<'valid> {
    fn render(&self) -> String {return format!(r"\left| {}\right| ", self.value.render())}
}

//> 5ºLEVEL -> UNDEFINED
impl LaTeX for Undefined {
    fn render(&self) -> String {return String::from(r"\left. ?\right. ")}
}

//> 5ºLEVEL -> RATIONAL
impl<'valid> LaTeX for Rational<'valid> {
    fn render(&self) -> String {return self.number.to_string()}
}

//> 5ºLEVEL -> CALL
impl<'valid> LaTeX for Call<'valid> {
    fn render(&self) -> String {return format!(
        r"{}\left( {}\right) ", 
        self.to.render(), 
        self.with.iter().map(LaTeX::render).collect::<Vec<String>>().join(",")
    )}
}