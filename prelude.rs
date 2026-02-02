//^
//^ PRELUDE
//^

//> PRELUDE -> STD
pub use std::{
    env::args,
    path::PathBuf,
    fs::{
        read_to_string,
        write
    },
    time::Instant,
    process::exit,
    error::Error,
    fmt,
    collections::{
        HashMap,
        HashSet,
        VecDeque
    },
    sync::LazyLock
};

//> PRELUDE -> ENTRY
pub use crate::{
    VERSION,
    entry::{
        Argument,
        File,
        Flag,
        Alias,
        Target
    },
    dev::wrapper as wrapperDev
};

//> PRELUDE -> COLORED
pub use colored::Colorize;

//> PRELUDE -> REGEX
pub use regex::bytes::{
    Regex,
    RegexSet
};

//> PRELUDE -> INDEXMAP
pub use indexmap::IndexMap;

//> PRELUDE -> AHASH
pub use ahash::{
    AHashMap,
    AHashSet
};

//> PRELUDE -> SMALLVEC
pub use smallvec::{
    SmallVec
};