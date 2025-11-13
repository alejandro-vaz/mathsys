//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;


//^
//^ INFINITY
//^

//> INFINITY -> STRUCT
#[derive(Clone)]
pub struct _Infinity {
    pub negative: bool
}

//> INFINITY -> IMPLEMENTATION
impl crate::runtime::Id for _Infinity {const ID: &'static str = "_Infinity";} 
impl crate::runtime::Value for _Infinity {
    fn id(&self) -> &'static str {"_Infinity"}
    fn ctrlcv(&self) -> crate::Box<dyn crate::runtime::Value> {return crate::Box::new(self.clone())}
    fn locale(&self, code: u8) -> () {match code {
        _ => {crate::stdout::crash(crate::stdout::Code::LocaleNotFound)}
    }}
} impl _Infinity {}