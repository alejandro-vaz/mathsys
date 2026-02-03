//^
//^ PRELUDE
//^

//> PRELUDE -> STD
pub use std::{
    env::args as getArguments,
    path::PathBuf as FilePath,
    fs::{
        read_to_string as read,
        write
    },
    time::Instant as Time,
    process::exit,
    error::Error,
    fmt as format,
    collections::{
        HashMap as Map,
        HashSet as Set,
        VecDeque as Deque
    },
    thread::spawn,
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
pub use colored::Colorize as Colored;

//> PRELUDE -> REGEX
pub use regex::bytes::{
    Regex,
    RegexSet
};

//> PRELUDE -> INDEXMAP
pub use indexmap::IndexMap;

//> PRELUDE -> AHASH
pub use ahash::{
    AHashMap as FastMap,
    AHashSet as FastSet
};

//> PRELUDE -> SMALLVEC
pub use smallvec::{
    SmallVec
};

//> PRELUDE -> NUM_CPUS
pub use num_cpus::{
    get as threadCount
};