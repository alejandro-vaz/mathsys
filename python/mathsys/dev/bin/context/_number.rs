//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;


//^
//^ NUMBER
//^

//> NUMBER -> STRUCT
#[derive(Clone)]
pub struct _Number {
    pub value: u32,
    pub shift: u8
}

//> NUMBER -> IMPLEMENTATION
impl crate::runtime::Id for _Number {const ID: &'static str = "_Number";} 
impl crate::runtime::Value for _Number {
    fn id(&self) -> &'static str {"_Number"}
    fn ctrlcv(&self) -> crate::Box<dyn crate::runtime::Value> {return crate::Box::new(self.clone())}
    fn locale(&self, code: u8) -> () {match code {
        _ => {crate::stdout::crash(crate::stdout::Code::LocaleNotFound)}
    }}
} impl _Number {}