//^
//^ HEAD
//^

//> HEAD -> STD
use std::sync::LazyLock;

//> HEAD -> LIBUTILS
use libutils::ebnftobnf::{
    Settings,
    reduce
};

//> HEAD -> SUPER
use super::{
    ebnf::EBNF,
    constants::{
        DELIMITER,
        TEMPORAL
    }
};


//^
//^ BNF
//^

//> BNF -> STATIC
pub static BNF: LazyLock<String> = LazyLock::new(|| reduce(EBNF, Settings {
    keep_comments: false,
    keep_empty_lines: false,
    delimiter: DELIMITER,
    start_rule: Some("Start"),
    temporal_production_character: TEMPORAL,
    ..
}));