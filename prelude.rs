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
        write as writeFile,
        read_dir as readDir
    },
    time::{
        Instant as Time,
        Duration
    },
    error::Error,
    process::ExitCode,
    fmt::{
        Formatter,
        Display,
        Result as Rst,
        Debug
    },
    collections::{
        HashMap as Map,
        HashSet as Set,
        VecDeque as Deque
    },
    sync::LazyLock,
    mem::take
};

//> PRELUDE -> COLORED
pub(super) use colored::Colorize as Colored;

//> PRELUDE -> REGEX
pub(super) use regex::bytes::{
    Regex,
    RegexSet,
    Captures
};

//> PRELUDE -> INDEXMAP
pub(super) use indexmap::IndexMap;

//> PRELUDE -> AHASH
pub(super) use ahash::{
    AHashMap as FastMap,
    AHashSet as FastSet
};

//> PRELUDE -> SMALLVEC
pub(super) use smallvec::{
    SmallVec
};

//> PRELUDE -> RUSTC_VERSION_RUNTIME
pub use rustc_version_runtime::{
    version as rustcv,
    Version
};

//> PRELUDE -> STRUM_MACROS
pub(super) use strum_macros::{
    AsRefStr,
    EnumString
};

//> PRELUDE -> ENUM_DISPATCH
pub(super) use enum_dispatch::enum_dispatch as dispatch;