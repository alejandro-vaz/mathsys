//^
//^ HEAD
//^

//> HEAD -> LIBUTILS
use libutils::issuing::Issue;

//> HEAD -> CORE
use core::num::NonZero;


//^
//^ FAILURE
//^

//> FAILURE -> ENUM
pub enum Failure<'valid> {
    UnknownToken {
        line: NonZero<usize>,
        column: NonZero<usize>
    },
    CircularImport {
        from: &'valid str,
        to: &'valid str
    }
}

//> FAILURE -> INTO ISSUE
impl<'valid> Into<Issue> for Failure<'valid> {
    fn into(self) -> Issue {return match self {
        Failure::UnknownToken {line, column} => Issue {
            name: "Unknown token",
            description: Some(format!("Unknown token at {line}:{column}")),
            ..
        },
        Failure::CircularImport {from, to} => Issue {
            name: "Circular import detected",
            description: Some(format!(
                "Circular import detected between {from:?} and {to:?}"
            )),
            ..
        }
    }.assert_normal()}
}