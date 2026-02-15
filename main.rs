//^
//^ HEAD
//^

//> HEAD -> FLAGS
#![allow(unused_variables)]
#![allow(nonstandard_style)]

//> HEAD -> LOCAL
use mathsys::{
    TARGETS,
    help,
    version,
    Transformers,
    prelude::{ARCH, OS, rustcv, getArguments},
    tokenizer::tokenizer::MAXLEN,
    issues::Issue,
    entry::{Argument, File, Flag, Target, Alias},
    tokens,
    length,
    check,
    ast,
    latex
};

//> TARGETS -> WRAPPER
pub fn main() -> Result<(), Issue> {
    let mut transformers = Transformers::new(&getArguments().skip(1).map(|argument| match &argument {
        file if file.split(".").last().unwrap() == "msm" => Argument::File(File {
            name: argument.into()
        }),
        flag if flag.starts_with("--") => Argument::Flag(Flag {
            value: argument.chars().skip(2).collect()
        }),
        alias if alias.starts_with("-") => Argument::Alias(Alias {
            letters: argument.chars().skip(1).collect()
        }),
        target => Argument::Target(Target {
            name: argument
        }),
    }).collect::<Vec<Argument>>())?;
    match &transformers.settings.target.clone().unwrap_or(Target {name: String::from("help")}).name as &str {
        target if target == TARGETS[0].0 => help(&mut transformers)?,
        target if target == TARGETS[1].0 => println!("Running Mathsys v{} on {}/{}/{}", version(&mut transformers)?, OS, ARCH, rustcv()),
        target if target == TARGETS[2].0 => println!("{:#?}", tokens(&mut transformers)?),
        target if target == TARGETS[3].0 => {let len = length(&mut transformers)?; println!("Token length: {len} / {MAXLEN} ({}%)", len as f32 / MAXLEN as f32 * 100.0)},
        target if target == TARGETS[4].0 => println!("{}", {check(&mut transformers)?; "Valid"}),
        target if target == TARGETS[5].0 => println!("{:#?}", ast(&mut transformers)?),
        target if target == TARGETS[6].0 => println!("{}", latex(&mut transformers)?),
        other => return Err(Issue::UnknownTarget(other.to_string()))
    };
    if transformers.settings.noise.verbose() {println!("Execution time: {:?}", transformers.time.elapsed())}
    return Ok(());
}