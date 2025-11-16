//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::runtime::Id;


//^
//^ INFINITY
//^

//> INFINITY -> STRUCT
#[derive(Clone)]
pub struct _Infinity {
    pub negative: bool
}

//> INFINITY -> IMPLEMENTATION
impl Id for _Infinity {const ID: &'static str = "_Infinity";} 
impl Value for _Infinity {
    fn id(&self) -> &'static str {return Self::ID}
    fn info(&self) -> () {
        crate::stdout::debug(&crate::format!("> negative = {}", self.negative));
    }
    fn ctrlcv(&self) -> crate::Box<dyn Value> {self.genlocale(0); return crate::Box::new(self.clone())}
    fn equiv(&self, to: crate::Box<dyn Value>) -> bool {self.genlocale(1); return match to.id() {
        "_Infinity" => {
            let value = crate::runtime::downcast::<crate::_Infinity>(&*to);
            self.negative == value.negative
        },
        "_Nexists" => false,
        "_Number" => false,
        "_Undefined" => false,
        "_Variable" => false,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn summation(&mut self, mut to: crate::Box<dyn Value>, inverse: bool, selfinverse: bool) -> crate::Box<dyn Value> {
        self.genlocale(2);
        if selfinverse {self.negate()}; 
        return match to.id() {
            "_Infinity" => {
                let value = crate::runtime::mutcast::<crate::_Infinity>(&mut *to);
                if inverse {value.negate()}
                if self.negative != value.negative {
                    let result = crate::_Undefined {};
                    result.ctrlcv()
                } else {self.ctrlcv()}
            },
            "_Nexists" => self.ctrlcv(),
            "_Number" => self.ctrlcv(),
            "_Undefined" => to.ctrlcv(),
            "_Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
            other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
        }
    }
    fn locale(&self, code: u8) -> () {match code {
        0 => crate::stdout::trace(&crate::format!(
            "Inverting sign of infinite to be {}",
            if !self.negative {"positive"} else {"negative"}
        )),
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
} impl _Infinity {
    pub fn negate(&mut self) -> () {self.locale(0); self.negative = !self.negative}
}