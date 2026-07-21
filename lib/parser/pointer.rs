//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::parsed::Parsed;


//^ 
//^ POINTER
//^ 

//> POINTER -> STRUCT
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Pointer<'valid> {
    pub parsed: Parsed<'valid>,
    pub start: usize,
    pub end: usize
}