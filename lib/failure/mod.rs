//^
//^ HEAD
//^

//> HEAD -> LIBUTILS
use libutils::issuing::Issue;


//^
//^ FAILURE
//^

//> FAILURE -> ENUM
pub enum Failure<'valid> {
    UnknownToken {
        index: usize
    },
    CircularImport {
        from: &'valid str,
        to: &'valid str
    }
}

//> FAILURE -> TOISSUE
impl<'valid> Into<Issue> for Failure<'valid> {
    fn into(self) -> Issue {return match self {
        Failure::UnknownToken {index} => Issue {
            name: "Unknown token",
            description: Some(format!("Unknown token at index {index}")),
            ..
        },
        Failure::CircularImport {from, to} => Issue {
            name: "Circular import detected",
            description: Some(format!("Circular import detected between {from:?} and {to:?}")),
            ..
        }
    }}
}