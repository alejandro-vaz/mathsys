//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Error, 
    Display, 
    Formatter, 
    Rst, 
    Colored, 
    currentDir, 
    readDir, 
    AsRefStr, 
    Debug
};

//> HEAD -> LOCAL
use super::{
    tokenizer::MAXLEN,
    entry::{
        Alias,
        Flag,
        Target
    }
};


//^
//^ DEFAULTS
//^

//> DEFAULTS -> STYLE
enum Style {
    Code,
    Place,
    Object,
    Target,
    Flag,
    Alias,
    Issue
}

//> DEFAULTS -> COLOR
fn color(string: impl Colored, style: Style) -> String {return match style {
    Style::Code => string.truecolor(0xAA, 0xAA, 0xAA),
    Style::Place => string.cyan(),
    Style::Object => string.purple().bold(),
    Style::Target => string.green(),
    Style::Flag => string.blue(),
    Style::Alias => string.yellow(),
    Style::Issue => string.red()
}.to_string()}


//^
//^ ISSUES
//^

//> ISSUES -> ALL
#[derive(AsRefStr, Debug)]
pub enum Issue {
    UnknownToken {
        line: u32,
        column: u32,
        code: String
    },
    UnknownArgument(String),
    MissingFile,
    InputTooLong,
    UnknownFlag(String),
    UnknownAliasCharacter {
        alias: Alias,
        at: usize
    },
    GetHelp,
    SyntaxError,
    FileNotFound(String)
} impl Issue {
    pub fn consume(self) -> u8 {println!("{self}"); &self as *const Issue as u8}
    fn content(&self) -> String {match self {
        Issue::UnknownToken {line, column, code} => format!(
            "Unknown token at line {},\n\n    {}\n    {}",
            color(&line.to_string() as &str, Style::Place),
            color(code as &str, Style::Code),
            " ".repeat(*column as usize - 1) + &color("^", Style::Place)
        ),
        Issue::UnknownArgument(argument) => format!(
            "Unknown argument {}.\n\nAvailable targets:\n{}",
            color(argument as &str, Style::Code),
            TARGETS.iter().map(|target| "- ".to_string() + &color(target.0.as_ref(), Style::Target) + ": " + target.1).collect::<Vec<String>>().join("\n"),
        ),
        Issue::MissingFile => format!(
            "Missing input file. Files available in {}:\n{}",
            color(currentDir().unwrap().to_str().unwrap(), Style::Place),
            readDir(currentDir().unwrap()).unwrap().into_iter().filter_map(|entry| if let Ok(thing) = entry && thing.path().is_file() {Some(thing.path())} else {None}).filter_map(|file| if let Some(extension) = file.extension() && extension.to_str().unwrap() == "msm" {Some("- ".to_string() + &color(file.strip_prefix(currentDir().unwrap()).unwrap().to_str().unwrap(), Style::Place))} else {None}).collect::<Vec<String>>().join("\n")
        ),
        Issue::InputTooLong => format!(
            "Input size exceeds limit of {MAXLEN}."
        ),
        Issue::UnknownFlag(flag) => format!(
            "Unknown flag provided: {}\n\nValid flags and aliases:\n{}",
            color(&("--".to_string() + flag.as_ref()) as &str, Style::Flag),
            FLAGLIASES.map(|tuple| "- ".to_string() + &color(&('-'.to_string() + &tuple.0.to_string()) as &str, Style::Alias) + ", " + &color(&("--".to_string() + &tuple.1.as_ref()) as &str, Style::Flag) + ": " + tuple.2).join("\n")
        ),
        Issue::UnknownAliasCharacter {alias, at} => format!(
            "Unknown alias character provided: {}\n{}\n\nValid flags and aliases:\n{}",
            color(&('-'.to_string() + &alias.0.iter().collect::<String>()) as &str, Style::Alias),
            " ".repeat(35 + at) + &color("^", Style::Place),
            FLAGLIASES.map(|tuple| "- ".to_string() + &color(&('-'.to_string() + &tuple.0.to_string()) as &str, Style::Alias) + ", " + &color(&("--".to_string() + &tuple.1.as_ref()) as &str, Style::Flag) + ": " + tuple.2).join("\n")
        ),
        Issue::GetHelp => format!(
            "Available targets:\n{}\n\nAvailable flags and aliases:\n{}",
            TARGETS.iter().map(|target| "- ".to_string() + &color(target.0.as_ref(), Style::Target) + ": " + target.1).collect::<Vec<String>>().join("\n"),
            FLAGLIASES.map(|tuple| "- ".to_string() + &color(&('-'.to_string() + &tuple.0.to_string()) as &str, Style::Alias) + ", " + &color(&("--".to_string() + &tuple.1.as_ref()) as &str, Style::Flag) + ": " + tuple.2).join("\n")
        ),
        Issue::SyntaxError => format!(
            "Syntax error."
        ),
        Issue::FileNotFound(name) => format!(
            "File {} not found.\n\nAvailable files in {}:\n{}",
            color(&name as &str, Style::Place),
            color(currentDir().unwrap().to_str().unwrap(), Style::Place),
            readDir(currentDir().unwrap()).unwrap().into_iter().filter_map(|entry| if let Ok(thing) = entry && thing.path().is_file() {Some(thing.path())} else {None}).filter_map(|file| if let Some(extension) = file.extension() && extension.to_str().unwrap() == "msm" {Some("- ".to_string() + &color(file.strip_prefix(currentDir().unwrap()).unwrap().to_str().unwrap(), Style::Place))} else {None}).collect::<Vec<String>>().join("\n")
        )
    }}
} impl Display for Issue {fn fmt(&self, formatter: &mut Formatter) -> Rst {write!(formatter, "{}", format!(
    "{} {} {}{}{}{}", 
    color("Raised", Style::Issue), 
    color(self.as_ref(), Style::Object),
    color("issue:", Style::Issue), 
    color("\n>\n> ", Style::Issue), 
    self.content().replace("\n", &color("\n> ", Style::Issue)), 
    color("\n>\n", Style::Issue)
))}} impl Error for Issue {}

//> ISSUE -> TARGETS
static TARGETS: [(Target, &'static str); 6] = [
    (Target::Help, "show this informative menu"),
    (Target::Version, "check current Mathsys version"),
    (Target::Tokens, "tokenize the input file"),
    (Target::Check, "validate the semantics"),
    (Target::Ast, "build the abstract syntax tree"),
    (Target::Latex, "show the latex program equivalency")
];

//> ISSUE -> FLAGS AND ALIASES
static FLAGLIASES: [(char, Flag, &'static str); 2] = [
    ('q', Flag::Quiet, "silence the output"),
    ('v', Flag::Verbose, "increase program verbosity")
];