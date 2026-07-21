//^
//^ HEAD
//^

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::Report,
    systemstd::{
        System,
        OpenMode
    }
};

//> HEAD -> ELSA
use elsa::FrozenMap;


//^
//^ HANDLER
//^

//> HANDLER -> CACHE
thread_local! {
    static CACHE: &'static FrozenMap<&'static str, Vec<u8>> = Box::leak(Box::new(
        FrozenMap::<&'static str, Vec<u8>>::new()
    ));
}

//> HANDLER -> RESOLVE
pub fn resolve(filename: &'static str, report: Report<"Resolver">) -> &'static [u8] {
    return CACHE.with(|cache| match cache.get(filename) {
        Some(cached) => cached,
        None => cache.insert(filename,System::expect(System::expect(
            System::path(filename).file::<{OpenMode::Read}>(None),
            &*report
        ).read_bytes(), &*report))
    })
}