//^
//^ HEAD
//^

//> HEAD -> LIBUTILS
use libutils::issuing::Issue;


//>
//^ INTERFACEERROR
//^

//> INTERFACEERROR -> ENUM
pub enum InterfaceError {
    UnknownTarget {
        name: &'static str
    },
    TargetNotProvided,
    IncorrectLatexArguments
}

//> INTERFACEERROR -> INTO ISSUE
impl Into<Issue> for InterfaceError {
    fn into(self) -> Issue {return match self {
        InterfaceError::UnknownTarget {name} => Issue {
            name: "unknown target",
            description: Some(format!("unknown target found: {name:?}")),
            ..
        },
        InterfaceError::TargetNotProvided => Issue {
            name: "target not provided",
            description: Some(String::from("interpreter target was not provided")),
            ..
        },
        InterfaceError::IncorrectLatexArguments => Issue {
            name: "incorrect arguments for latex",
            description: Some(format!("usage: `mathsys latex (FILE)`")),
            ..
        }
    }.assert_normal()}
}