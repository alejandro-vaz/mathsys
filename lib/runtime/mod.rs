//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::failure::Failure;


//^
//^ RUNTIME
//^

//> RUNTIME -> TRAIT
pub const trait Runtime<'valid> {
    fn resolve(&'valid self, module: &'valid str) -> &'valid [u8];
    fn critical(&'valid self, failure: Failure<'valid>) -> !;
    fn error(&'valid self, failure: Failure<'valid>) -> ();
    fn warning(&'valid self, failure: Failure<'valid>) -> ();
}