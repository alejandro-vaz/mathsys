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
    fn critical(failure: Failure<'valid>) -> !;
    fn error(failure: Failure<'valid>) -> ();
    fn warning(failure: Failure<'valid>) -> ();
}