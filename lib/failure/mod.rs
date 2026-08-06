//^
//^ HEAD
//^

//> HEAD -> LIBUTILS
use libutils::issuing::Issue;

//> HEAD -> CRATE
use crate::tokenizer::position::Position;

//> HEAD -> CORE
use core::str::Utf8Error;

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;


//^
//^ FAILURE
//^

//> FAILURE -> ENUM
#[derive(EnumAsInner)]
pub enum Failure<'valid> {
    UnknownToken {
        filename: &'valid str,
        position: Position
    },
    CircularImport {
        from: &'valid str,
        to: &'valid str
    },
    UnmatchedModuleDelimiter {
        filename: &'valid str,
        start: Position
    },
    IrregularText {
        filename: &'valid str,
        starting: Position,
        error: Utf8Error
    },
    TokenNotFound,
    CouldntParseStatement,
    CouldntParseFactor,
    CouldntParseValue,
    TokenStreamDepleted,
    CouldntParseMore
}

//> FAILURE -> INTO ISSUE
impl<'valid> Into<Issue> for Failure<'valid> {
    fn into(self) -> Issue {return match self {
        Failure::UnknownToken {filename, position} => Issue {
            name: "Unknown token",
            description: Some(format!(
                "Unknown token at {filename:?} ({}:{})",
                position.line,
                position.column
            )),
            ..
        },
        Failure::CircularImport {from, to} => Issue {
            name: "Circular import detected",
            description: Some(format!(
                "Circular import detected between {from:?} and {to:?}"
            )),
            ..
        },
        Failure::UnmatchedModuleDelimiter {filename, start} => Issue {
            name: "unmatched token delimiter",
            description: Some(format!(
                "failed to found module delimiter in {filename:?} ({}:{})",
                start.line,
                start.column
            )),
            ..
        },
        Failure::IrregularText {filename, starting, error} => Issue {
            name: "irregular text detected",
            traceback: Some(error.to_string()),
            description: Some(format!(
                "Non-UTF-8 text detected at {filename:?} ({}:{})",
                starting.line,
                starting.column
            )),
            ..
        },
        Failure::TokenNotFound => Issue {
            name: "Failed to find token",
            ..
        },
        Failure::CouldntParseStatement => Issue {
            name: "Failed to parse statement",
            ..
        },
        Failure::CouldntParseFactor => Issue {
            name: "Failed to parse factor",
            ..
        },
        Failure::CouldntParseValue => Issue {
            name: "Failed to parse value",
            ..
        },
        Failure::TokenStreamDepleted => Issue {
            name: "Token stream depleted",
            ..
        },
        Failure::CouldntParseMore => Issue {
            name: "Failed to parse more",
            ..
        }
    }.assert_normal()}
}