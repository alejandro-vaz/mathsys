//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;


//^
//^ NEXISTS
//^

//> NEXISTS -> CONSTRUCT
#[derive(Clone)]
pub struct _Nexists {}

//> NEXISTS -> IMPLEMENTATION
impl crate::runtime::Id for _Nexists {const ID: &'static str = "_Nexists";} 
impl crate::runtime::Value for _Nexists {
    fn id(&self) -> &'static str {"_Nexists"}
    fn ctrlcv(&self) -> crate::Box<dyn crate::runtime::Value> {return crate::Box::new(self.clone())}
    fn locale(&self, code: u8) -> () {match code {
        _ => {crate::stdout::crash(crate::stdout::Code::LocaleNotFound)}
    }}
}