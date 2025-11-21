//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::runtime::Id;


//^
//^ TENSOR
//^

//> TENSOR -> CONSTRUCT
#[derive(Clone)]
pub struct Tensor {}

//> TENSOR -> IMPLEMENTATION
impl Id for Tensor {const ID: &'static str = "Tensor";} 
impl Value for Tensor {
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
        "Tensor" => true,
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
            "Tensor" => self.ctrlcv(),
            "Undefined" => to,
            "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
            other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
        }
    }
    fn locale(&self, code: u8) -> () {match code {
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
}