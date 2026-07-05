//^
//^ HEAD
//^

//> HEAD -> LIBUTILS
use libutils::issue::{
    Issue,
    Severity
};


//^
//^ FAILURE
//^

//> FAILURE -> ENUM
pub enum Failure<'valid> {
    UnknownToken,
    UnknownFile(&'valid str),
    IncorrectArgumentAmount(usize),
    IncorrectArgumentDistribution,
    CircularImport(&'valid str, &'valid str)
}

//> FAILURE -> TOISSUE
impl<'valid> Into<Issue> for Failure<'valid> {
    fn into(self) -> Issue {return match self {
        Failure::UnknownToken => Issue {
            name: "Unknown token",
            description: None,
            severity: Severity::Critical
        },
        Failure::UnknownFile(file) => Issue { 
            name: "Unknown file", 
            description: Some(format!("Failed to load file {file:?}")),
            severity: Severity::Critical
        },
        Failure::IncorrectArgumentAmount(amount) => Issue {
            name: "Incorrect argument amount",
            description: Some(format!("Two arguments were required but {amount} were supplied")),
            severity: Severity::Critical
        },
        Failure::IncorrectArgumentDistribution => Issue {
            name: "Incorrect argument distribution",
            description: None,
            severity: Severity::Critical
        },
        Failure::CircularImport(main, module) => Issue {
            name: "Circular import detected",
            description: Some(format!("Circular import detected between {main:?} and {module:?}")),
            severity: Severity::Critical
        }
    }}
}