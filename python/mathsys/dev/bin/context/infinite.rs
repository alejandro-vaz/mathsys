//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::runtime::Id;


//^
//^ INFINITE
//^

//> INFINITE -> STRUCT
#[derive(Clone)]
pub struct Infinite {
    pub negative: bool
}

//> INFINITE -> IMPLEMENTATION
impl Id for Infinite {const ID: &'static str = "Infinite";} 
impl Value for Infinite {
    fn id(&self) -> &'static str {return Self::ID}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > negative = {}", 
        self.id(), 
        self.negative
    ))}
    fn ctrlcv(&self) -> crate::Box<dyn Value> {self.genlocale(0); return crate::Box::new(self.clone())}
    fn equiv(&self, mut to: crate::Box<dyn Value>) -> bool {self.genlocale(1); return match to.id() {
        "Infinite" => {
            let value = crate::runtime::mutcast::<crate::Infinite>(&mut *to);
            self.negative == value.negative
        },
        "Nexists" => false,
        "Number" => false,
        "Tensor" => false,
        "Undefined" => false,
        "Variable" => false,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn summation(&mut self, mut to: crate::Box<dyn Value>, inverse: bool, selfinverse: bool) -> crate::Box<dyn Value> {
        self.genlocale(2);
        if selfinverse {self.negate()}; 
        return match to.id() {
            "Infinite" => {
                let value = crate::runtime::mutcast::<crate::Infinite>(&mut *to);
                if inverse {value.negate()}
                if self.negative != value.negative {crate::Box::new(crate::Undefined {})} else {self.ctrlcv()}
            },
            "Nexists" => self.ctrlcv(),
            "Number" => self.ctrlcv(),
            "Undefined" => to,
            "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
            other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
        }
    }
    fn locale(&self, code: u8) -> () {match code {
        0 => crate::stdout::trace(&crate::format!(
            "Inverting sign of infinite to be {}",
            if !self.negative {"positive"} else {"negative"}
        )),
        1 => crate::stdout::trace("Calculating absolute value"),
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
} impl Infinite {
    pub fn negate(&mut self) -> () {self.locale(0); self.negative = !self.negative}
    pub fn abs(&mut self) -> () {self.locale(1); self.negative = false}
}