//^
//^ HEAD
//^

//> HEAD -> LIBUTILS
use libutils::issuing::Issue;

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;


//^
//^ FAILURE
//^

//> FAILURE -> ENUM
#[derive(EnumAsInner)]
pub enum Failure<'valid> {
    CircularImport {
        from: &'valid str,
        to: &'valid str
    },
    TokenNotFound,
    CouldntParseStatement,
    CouldntParseFactor,
    CouldntParseValue,
    TokenStreamDepleted,
    CouldntParseMore,
    UnfinishedInputParse,
    NonUtf8Sequence
}

//> FAILURE -> INTO ISSUE
impl<'valid> Into<Issue> for Failure<'valid> {
    fn into(self) -> Issue {return match self {
        Failure::CircularImport {from, to} => Issue {
            name: "circular import detected",
            description: Some(format!(
                "Circular import detected between {from:?} and {to:?}"
            )),
            ..
        },
        Failure::TokenNotFound => Issue {
            name: "failed to find token",
            ..
        },
        Failure::CouldntParseStatement => Issue {
            name: "failed to parse statement",
            ..
        },
        Failure::CouldntParseFactor => Issue {
            name: "failed to parse factor",
            ..
        },
        Failure::CouldntParseValue => Issue {
            name: "failed to parse value",
            ..
        },
        Failure::TokenStreamDepleted => Issue {
            name: "token stream depleted",
            ..
        },
        Failure::CouldntParseMore => Issue {
            name: "failed to parse more",
            ..
        },
        Failure::UnfinishedInputParse => Issue {
            name: "failed to parse whole input",
            ..
        },
        Failure::NonUtf8Sequence => Issue {
            name: "tried to parse a non-UTF8 sequence",
            ..
        }
    }.assert_normal()}
}