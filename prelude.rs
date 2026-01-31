//^
//^ PRELUDE
//^

//> PRELUDE -> STD
pub use std::{
    env::args,
    path::PathBuf,
    future::Future,
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

//> PRELUDE -> TOKIO
pub use tokio::{
    fs::{
        read_to_string as read,
        write
    },
    runtime::Builder
};

//> PRELUDE -> COLORED
pub use colored::Colorize;

//> PRELUDE -> REGEX
pub use regex::bytes::Regex;