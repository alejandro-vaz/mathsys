//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;


//^
//^ EQUATION
//^

//> EQUATION -> STRUCT
pub struct Equation {
    left: u32,
    right: u32
}

//> EQUATION -> IMPLEMENTATION
impl crate::converter::Class for Equation {
    fn name(&self) -> &'static str {"Equation"}
    fn locale(&self, code: u8) -> () {match code {
        0 => {crate::ALLOCATOR.tempSpace(|| {crate::stdout::debug(&crate::format!(
            "Sides of the equation have IDs {} and {}",
            self.left,
            self.right
        ))})},
        1 => {crate::stdout::space("[Equation] Verifying equality of both sides")},
        2 => {crate::stdout::debug("Sides of the equation are equal")},
        3 => {crate::stdout::alert("Sides of the equation are not equal")},
        _ => {crate::stdout::crash(crate::stdout::Code::LocaleNotFound)}
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        self.locale(0);
        context.process(self.left);
        context.process(self.right);
        self.locale(1);
        let left = context.read(self.left);
        let right = context.read(self.right);
        if left.equiv(right) {self.locale(2)} else {self.locale(3)}
        return crate::Box::new(crate::_Undefined {});
    }
} impl Equation {
    pub fn new(left: u32, right: u32) -> Self {return Equation {
        left: left,
        right: right
    }}
}