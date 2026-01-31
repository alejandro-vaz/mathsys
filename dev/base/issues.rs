//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    exit, Error, fmt, Colorize
};

//> HEAD -> LOCAL
use super::tokenizer::MAXLEN;
use super::super::TARGETS;


//^
//^ DEFAULTS
//^

//> DEFAULTS -> STYLE
enum Style {
    Code,
    Place,
    Object,
    Target
}

//> DEFAULTS -> COLOR
fn color<Colorable: Colorize>(string: Colorable, style: Style) -> String {return (match style {
    Style::Code => string.truecolor(0xAA, 0xAA, 0xAA),
    Style::Place => string.cyan(),
    Style::Object => string.purple(),
    Style::Target => string.green()
}).to_string()}


//^
//^ ISSUES
//^

//> ISSUES -> ISSUE
#[derive(Debug)]
pub struct Issue {
    name: &'static str,
    content: String
} impl Issue {
    pub fn is(name: &'static str, content: String) -> Self {Self {name: name, content: content}}
    pub fn consume(self) -> ! {println!("{self}"); exit(1)}
} 
impl fmt::Display for Issue {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    let base = format!("{} {} {}{}{}{}", "Raised".red(), color(self.name, Style::Object), "issue:".red(), "\n>\n> ".red(), self.content.replace("\n", &"\n> ".red().to_string()), "\n>\n".red());
    write!(formatter, "{base}")
}}
impl Error for Issue {}

//> ISSUES -> UNKNOWN TOKEN
pub fn unknownToken(line: u16, column: u16, code: &str) -> Issue {Issue::is("unknownToken", format!(
    "Unknown token at line {}\n\n    {}\n    {}",
    color(line.to_string().as_str(), Style::Place),
    color(code, Style::Code),
    " ".repeat((column - 1) as usize) + color("^", Style::Place).as_str()
))}

//> ISSUES -> UNKNOWN TARGET
pub fn unknownTarget(unknown: &str) -> Issue {Issue::is("unknownTarget", format!(
    "Unknown target {}.\n\nAvailable targets:\n{}",
    color(unknown, Style::Target),
    TARGETS.map(|target| "- ".to_string() + target).join("\n")
))}

//> ISSUES -> NO FILE PROVIDED
pub fn noFileProvided() -> Issue {Issue::is("noFileProvided" ,format!(
    "No input file provided."
))}

//> ISSUES -> NO TARGET PROVIDED
pub fn noTargetProvided() -> Issue {Issue::is("noTargetProvided", format!(
    "Available targets:\n{}",
    TARGETS.map(|target| "- ".to_string() + target).join("\n")
))}

//> ISSUES -> INPUT TOO LONG
pub fn inputTooLong(length: usize) -> Issue {Issue::is("inputTooLong", format!(
    "Input size is too long ({length}/{MAXLEN})"
))}