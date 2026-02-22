//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    LazyLock, 
    Regex, 
    Map, 
    Captures
};


//^
//^ LATEX
//^

//> LATEX -> VARIABLES
static VARIABLES: LazyLock<Map<&'static str, &'static str>> = LazyLock::new(|| Map::from([
    //= VARIABLES -> 7 GREEK
    ("epsilon", r"\epsilon "),
    ("Epsilon", r"E"),
    ("omicron", r"\omicron "),
    ("Omicron", r"O"),
    ("upsilon", r"\upsilon "),
    ("Upsilon", r"\Upsilon "),
    //= VARIABLES -> 6 GREEK
    ("lambda", r"\lambda "),
    ("Lambda", r"\Lambda "),
    //= VARIABLES -> 5 GREEK
    ("alpha", r"\alpha "),
    ("Alpha", r"A"),
    ("gamma", r"\gamma "),
    ("Gamma", r"\Gamma "),
    ("delta", r"\delta "),
    ("Delta", r"\Delta "),
    ("theta", r"\theta "),
    ("Theta", r"\Theta "),
    ("kappa", r"\kappa "),
    ("Kappa", r"K"),
    ("sigma", r"\sigma "),
    ("Sigma", r"\Sigma "),
    ("omega", r"\omega "),
    ("Omega", r"\Omega "),
    //= VARIABLES -> 5 DOTLESS
    ("imath", r"\imath "),
    ("jmath", r"\jmath "),
    //= VARIABLES -> 4 GREEK
    ("beta", r"\beta "),
    ("Beta", r"B"),
    ("zeta", r"\zeta "),
    ("Zeta", r"Z"),
    ("iota", r"\iota "),
    ("Iota", r"I"),
    //= VARIABLES -> 4 EXTRA
    ("hbar", r"\hbar "),
    //= VARIABLES -> 3 GREEK
    ("eta", r"\eta "),
    ("Eta", r"H"),
    ("rho", r"\rho "),
    ("Rho", r"P"),
    ("tau", r"\tau "),
    ("Tau", r"T"),
    ("phi", r"\phi "),
    ("Phi", r"\Phi "),
    ("chi", r"\chi "),
    ("Chi", r"X"),
    ("psi", r"\psi "),
    ("Psi", r"\Psi "),
    //= VARIABLES -> 3 EXTRA
    ("ell", r"\ell "),
    //= VARIABLES -> 2 GREEK
    ("mu", r"\mu "),
    ("Mu", r"M"),
    ("nu", r"\nu "),
    ("Nu", r"N"),
    ("xi", r"\xi "),
    ("Xi", r"\Xi "),
    ("pi", r"\pi "),
    ("Pi", r"\Pi "),
    //= VARIABLES -> 2 WEIERSTRASS
    ("wp", r"\wp "),
    //= VARIABLES -> 1 PERCENTAGE
    ("%", r"\% ")
]));

//> LATEX -> VARREGEX
static VARREGEX: LazyLock<Regex> = LazyLock::new(|| {
    let mut keys = VARIABLES.keys().copied().collect::<Vec<&'static str>>();
    keys.sort_by_key(|key| -(key.len() as isize));
    let pattern = format!(r"({})", keys.join("|"));
    Regex::new(&pattern).unwrap()
});

//> LATEX -> AUGMENT VARIABLES
pub fn augmentVariables(input: &str) -> String {return str::from_utf8(&VARREGEX.replace_all(input.as_bytes(), |captures: &Captures| VARIABLES.get(str::from_utf8(&captures[0]).unwrap()).unwrap())).unwrap().to_string()}