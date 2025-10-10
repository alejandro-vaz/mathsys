//
//  INFINITY
//

// INFINITY -> STRUCT
#[derive(Clone, Copy)]
pub struct _Infinity {}

// INFINITY -> IMPLEMENTATION
impl crate::runtime::Value for _Infinity {fn id(&self) -> &'static str {"Infinity"}} 
impl crate::runtime::Id for _Infinity {const ID: &'static str = "Infinity";} 
impl _Infinity {}