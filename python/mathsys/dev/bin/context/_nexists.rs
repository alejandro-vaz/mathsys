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
    fn id(&self) -> &'static str {crate::ALLOCATOR.tempSpace(|| {crate::stdout::trace(&crate::format!(
        "Selected element is of type {}",
        Self::ID
    ))}); return Self::ID}
    fn ctrlcv(&self) -> crate::Box<dyn Value> {return crate::Box::new(self.clone())}
    fn locale(&self, code: u8) -> () {match code {
        _ => {crate::stdout::crash(crate::stdout::Code::LocaleNotFound)}
    }}
    fn equiv(&self, to: crate::Box<dyn Value>) -> bool {self.id(); return match to.id() {
        "_Infinity" => to.equiv(self.ctrlcv()),
        "_Nexists" => false,
        "_Number" => false,
        "_Undefined" => false,
        "_Variable" => false,
        _ => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
}