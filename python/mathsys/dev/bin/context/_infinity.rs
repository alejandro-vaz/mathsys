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
    fn id(&self) -> &'static str {crate::ALLOCATOR.tempSpace(|| {crate::stdout::trace(&crate::format!(
        "Selected element is of type {}",
        Self::ID
    ))}); return Self::ID}
    fn ctrlcv(&self) -> crate::Box<dyn Value> {return crate::Box::new(self.clone())}
    fn locale(&self, code: u8) -> () {match code {
        _ => {crate::stdout::crash(crate::stdout::Code::LocaleNotFound)}
    }}
    fn equiv(&self, to: crate::Box<dyn Value>) -> bool {self.id(); return match to.id() {
        "_Infinity" => {
            let value = crate::runtime::downcast::<crate::_Infinity>(&*to);
            self.negative == value.negative
        }
        "_Nexists" => false,
        "_Number" => false,
        "_Undefined" => false,
        "_Variable" => false,
        _ => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
} impl _Infinity {
    pub fn change(&mut self) -> () {self.negative = !self.negative}
}