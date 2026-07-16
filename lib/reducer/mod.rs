//^
//^ HEAD
//^

//> HEAD -> MODULES
mod grammar;

//> HEAD -> GRAMMAR
use grammar::GRAMMAR;

//> HEAD -> LIBUTILS
use libutils::ebnftobnf::{
    reduce as convert,
    Settings,
    Delimiter
};

//> HEAD -> STD
use std::sync::LazyLock;


//^
//^ REDUCER
//^

//> REDUCER -> STATIC
static REDUCED: LazyLock<String> = LazyLock::new(|| convert(GRAMMAR, Settings {
    keep_comments: false,
    keep_empty_lines: false,
    delimiter: DELIMITER,
    start_rule: Some("Start"),
    ..
}));

//> REDUCER -> DELIMITER
pub static DELIMITER: Delimiter = Delimiter::SpaceThinArrowSpace;

//> REDUCER -> WIDTH
pub static WIDTH: usize = 49;

//> REDUCER -> FUNCTION
pub fn reduce() -> [&'static str; WIDTH] {
    return *REDUCED.split('\n').collect::<Vec<&'static str>>().as_array().unwrap();
}