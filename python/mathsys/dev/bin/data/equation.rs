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
        4 => {crate::stdout::alert("Impossible to determine equality")},
        _ => {crate::stdout::crash(crate::stdout::Code::LocaleNotFound)}
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        self.locale(0);
        context.process(self.left);
        context.process(self.right);
        self.locale(1);
        let left = context.read(self.left);
        match left.id() {
            "_Infinity" => self.caseInfinity(left, context),
            "_Number" => self.caseNumber(left, context),
            "_Undefined" => self.caseUndefined(left, context),
            _ => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
        };
        return crate::Box::new(crate::_Undefined {});
    }
} impl Equation {
    fn caseInfinity(&self, left: crate::Box<dyn crate::runtime::Value>, context: &mut crate::runtime::Context) -> () {
        let right = context.read(self.right);
        match right.id() {
            "_Infinity" => {
                let lvalue = crate::runtime::downcast::<crate::_Infinity>(&*left);
                let rvalue = crate::runtime::downcast::<crate::_Infinity>(&*right);
                if lvalue.negative == rvalue.negative {self.locale(2)} else {self.locale(3)}
            },
            "_Number" | "_Undefined" => self.locale(3),
            _ => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
        }
    }
    fn caseNumber(&self, left: crate::Box<dyn crate::runtime::Value>, context: &mut crate::runtime::Context) -> () {
        let right = context.read(self.right);
        match right.id() {
            "_Infinity" | "_Undefined" => self.locale(3),
            "_Number" => {
                let lvalue = crate::runtime::downcast::<crate::_Number>(&*left);
                let rvalue = crate::runtime::downcast::<crate::_Number>(&*right);
                if lvalue.value == rvalue.value && lvalue.shift == rvalue.shift {self.locale(2)} else {self.locale(3)}
            }
            _ => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
        }
    }
    fn caseUndefined(&self, left: crate::Box<dyn crate::runtime::Value>, context: &mut crate::runtime::Context) -> () {
        let right = context.read(self.right);
        match right.id() {
            "_Infinity" | "_Number" | "_Undefined" => self.locale(4),
            _ => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
        }
    }
    pub fn new(left: u32, right: u32) -> Self {return Equation {
        left: left,
        right: right
    }}
}