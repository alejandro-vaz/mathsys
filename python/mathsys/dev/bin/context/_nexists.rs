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
pub struct _Nexists {}

//> NEXISTS -> IMPLEMENTATION
impl Id for _Nexists {const ID: &'static str = "_Nexists";} 
impl Value for _Nexists {
    fn id(&self) -> &'static str {return Self::ID}
    fn ctrlcv(&self) -> crate::Box<dyn Value> {self.genlocale(0); return crate::Box::new(self.clone())}
    fn equiv(&self, to: crate::Box<dyn Value>) -> bool {self.genlocale(1); return match to.id() {
        "_Infinity" => to.equiv(self.ctrlcv()),
        "_Nexists" => false,
        "_Number" => false,
        "_Undefined" => false,
        "_Variable" => false,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn summation(&mut self, mut to: crate::Box<dyn Value>, inverse: bool, selfinverse: bool) -> crate::Box<dyn Value> {
        self.genlocale(2);
        return match to.id() {
            "_Infinity" => to.summation(self.ctrlcv(), false, inverse),
            "_Nexists" => self.ctrlcv(),
            "_Number" => to,
            "_Undefined" => to,
            "_Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
            other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
        }
    }
    fn locale(&self, code: u8) -> () {match code {
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
} impl _Nexists {}