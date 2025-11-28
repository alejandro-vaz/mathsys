//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::runtime::Id;


//^
//^ NEXISTS
//^

//> NEXISTS -> CONSTRUCT
#[derive(Clone)]
pub struct Nexists {}

//> NEXISTS -> IMPLEMENTATION
impl Id for Nexists {const ID: &'static str = "Nexists";} 
impl Value for Nexists {
    fn id(&self) -> &'static str {return Self::ID}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > ", 
        self.id()
    ))}
    fn ctrlcv(&self) -> crate::Box<dyn Value> {self.genlocale(0); return crate::Box::new(self.clone())}
    fn equiv(&self, mut to: crate::Box<dyn Value>) -> bool {self.genlocale(1); return match to.id() {
        "Infinite" => to.equiv(self.ctrlcv()),
        "Nexists" => false,
        "Number" => false,
        "Tensor" => false,
        "Undefined" => false,
        "Variable" => false,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn summation(&mut self, mut to: crate::Box<dyn Value>, inverse: bool, selfinverse: bool) -> crate::Box<dyn Value> {
        self.genlocale(2);
        return match to.id() {
            "Infinite" => to.summation(self.ctrlcv(), false, inverse),
            "Nexists" => self.ctrlcv(),
            "Number" => {
                let value = crate::runtime::mutcast::<crate::Number>(&mut *to);
                if inverse {value.negate()}
                value.ctrlcv()
            },
            "Tensor" => to,
            "Undefined" => to,
            "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
            other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
        }
    }
    fn locale(&self, code: u8) -> () {match code {
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
} impl Nexists {}