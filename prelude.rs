//^
//^ PRELUDE
//^

//> PRELUDE -> STD
pub use std::{
    env::{
        args as getArguments,
        consts::{
            OS,
            ARCH
        },
        current_dir as currentDir
    },
    path::PathBuf as FilePath,
    fs::{
        read_to_string as readFile,
        write,
        read_dir as readDir
    },
    time::Instant as Time,
    process::exit,
    error::Error,
    fmt::{
        Formatter,
        Display,
        Result as Rst
    },
    collections::{
        HashMap as Map,
        HashSet as Set,
        VecDeque as Deque
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

//> PRELUDE -> RUSTC_VERSION_RUNTIME
pub use rustc_version_runtime::version as rustcv;

//> PRELUDE -> STRUM_MACROS
pub use strum_macros::AsRefStr;