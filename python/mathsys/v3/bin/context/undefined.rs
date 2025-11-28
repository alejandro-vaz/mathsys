//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::runtime::Id;


//^
//^ UNDEFINED
//^

//> UNDEFINED -> STRUCT
#[derive(Clone)]
pub struct Undefined {}

//> UNDEFINED -> IMPLEMENTATION
impl Id for Undefined {const ID: &'static str = "Undefined";} 
impl Value for Undefined {
    fn id(&self) -> &'static str {return Self::ID}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > ", 
        self.id()
    ))}
    fn ctrlcv(&self) -> crate::Box<dyn Value> {self.genlocale(0); return crate::Box::new(self.clone())}
    fn equiv(&self, mut to: crate::Box<dyn Value>) -> bool {self.genlocale(1); return match to.id() {
        "Infinite" => to.equiv(self.ctrlcv()),
        "Nexists" => to.equiv(self.ctrlcv()),
        "Number" => to.equiv(self.ctrlcv()),
        "Tensor" => to.equiv(self.ctrlcv()),
        "Undefined" => false,
        "Variable" => false,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn summation(&mut self, mut to: crate::Box<dyn Value>, inverse: bool, selfinverse: bool) -> crate::Box<dyn Value> {
        self.genlocale(2);
        return match to.id() {
            "Infinite" => to.summation(self.ctrlcv(), false, inverse),
            "Nexists" => to.summation(self.ctrlcv(), false, inverse),
            "Number" => to.summation(self.ctrlcv(), false, inverse),
            "Tensor" => to.summation(self.ctrlcv(), false, inverse),
            "Undefined" => self.ctrlcv(),
            "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
            other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
        }
    }
    fn locale(&self, code: u8) -> () {match code {
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
} impl Undefined {}