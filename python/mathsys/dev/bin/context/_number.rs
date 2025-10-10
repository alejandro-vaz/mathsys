//
//  NUMBER
//

// NUMBER -> STRUCT
#[derive(Clone, Copy)]
pub struct _Number {
    pub value: u32,
    pub shift: u8
}

// NUMBER -> IMPLEMENTATION
impl crate::runtime::Value for _Number {fn id(&self) -> &'static str {"Number"}} 
impl crate::runtime::Id for _Number {const ID: &'static str = "Number";} 
impl _Number {}