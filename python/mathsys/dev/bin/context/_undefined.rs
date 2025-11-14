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
pub struct _Undefined {}

//> UNDEFINED -> IMPLEMENTATION
impl crate::runtime::Id for _Undefined {const ID: &'static str = "_Undefined";} 
impl Value for _Undefined {
    fn id(&self) -> &'static str {crate::ALLOCATOR.tempSpace(|| {crate::stdout::trace(&crate::format!(
        "Element is of type {}",
        Self::ID
    ))}); return Self::ID}
    fn ctrlcv(&self) -> crate::Box<dyn crate::runtime::Value> {return crate::Box::new(self.clone())}
    fn locale(&self, code: u8) -> () {match code {
        _ => {crate::stdout::crash(crate::stdout::Code::LocaleNotFound)}
    }}
} impl _Undefined {}