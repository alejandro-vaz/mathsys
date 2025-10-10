//
//  UNDEFINED
//

// UNDEFINED -> STRUCT
#[derive(Clone, Copy)]
pub struct _Undefined {}

// UNDEFINED -> IMPLEMENTATION
impl crate::runtime::Value for _Undefined {fn id(&self) -> &'static str {"Undefined"}} 
impl crate::runtime::Id for _Undefined {const ID: &'static str = "Undefined";} 
impl _Undefined {}