//^
//^ HEAD
//^

//> HEAD -> MATHSYS
use mathsys::{
    Runtime,
    Failure
};

//> HEAD -> LIBUTILS
use libutils::systemstd::{
    System,
    Handling,
    OpenMode
};

//> HEAD -> ELSA
use elsa::FrozenMap;


//^
//^ HANDLER
//^

//> HANDLER -> STRUCT
#[derive(Default)]
pub struct Handler<'valid> {
    cache: FrozenMap<&'valid str, Vec<u8>>
}

//> HANDLER -> RUNTIME
impl<'valid> Runtime<'valid> for Handler<'valid> {
    fn critical(&'valid self, failure: Failure<'valid>) -> ! {System::critical(failure, &[])}
    fn resolve(&'valid self, module: &'valid str) -> &'valid [u8] {
        return match self.cache.get(module) {
            Some(cached) => cached,
            None => self.cache.insert(module, System::expect(System::expect(
                System::path(module).file::<{OpenMode::Read}>(Handling::AssumeExists),
                &[]
            ).read_bytes(), &[]))
        }
    }
    fn error(&'valid self, failure: Failure<'valid>) -> () {System::error(failure, &[])}
    fn warning(&'valid self, failure: Failure<'valid>) -> () {System::error(failure, &[])}
}