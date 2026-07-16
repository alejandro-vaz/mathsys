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
            description: Some(format!("Unknown target found: {name:?}"))
        },
        InterfaceError::TargetNotProvided => Issue {
            name: "target not provided",
            description: Some(String::from("Interpreter target was not provided"))
        },
        Self::IncorrectLatexArguments => Issue {
            name: "incorrect arguments for latex",
            description: Some(format!("usage: `mathsys latex (FILE)`"))
        }
    }}
}