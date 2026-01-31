//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    HashMap, Regex, HashSet, LazyLock
};

//> HEAD -> LOCAL
use super::nonterminal::NonTerminals;
use super::tokenizer::{Kind, Token};


//^
//^ EBNF
//^

//> EBNF -> PATTERNS
static COLLAPSEREGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\(([^()]+)\)").unwrap());
static POSTFIXREGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?P<atom>\$[0-9]+|[A-Z][a-z]*[0-9]*|_?[A-Z]+)(?P<operator>[*+?])").unwrap());

//> EBNF -> EXTENSOR
struct Extensor {
    counter: u8,
    parentheses: HashMap<String, u8>,
    more: HashMap<String, u8>,
    multiple: HashMap<String, u8>,
    optional: HashMap<String, u8>,
} impl Extensor {
    pub fn new() -> Extensor {return Self {
        counter: 0,
        parentheses: HashMap::new(),
        more: HashMap::new(),
        multiple: HashMap::new(),
        optional: HashMap::new(),
    }}
    fn reset(&mut self) -> HashSet<String> {
        self.counter = 0;
        self.parentheses.clear();
        self.more.clear();
        self.multiple.clear();
        self.optional.clear();
        return HashSet::new();
    }
    pub fn run(&mut self, ebnf: &str) -> String {
        let mut out = self.reset();
        for line in ebnf.lines().map(|each| each.trim()) {
            if line.is_empty() || line.starts_with("//") {continue}
            let [rule, productions] = &line.splitn(2, "->").map(|part| part.trim().to_string()).collect::<Vec<String>>()[0..2] else {panic!("{line}")};
            let (body, rules) = self.expand(productions);
            out.insert(format!("{rule} -> {body}"));
            out.extend(rules);
        };
        let mut ordered = out.into_iter().collect::<Vec<String>>();
        ordered.sort_by(|first, second| {
            let [frule, srule] = [first, second].map(|each| each.split("->").next().unwrap().trim());
            let fkey = if frule.starts_with('$') {(1, frule[1..].parse::<usize>().unwrap())} else {(0, 0)};
            let skey = if srule.starts_with('$') {(1, srule[1..].parse::<usize>().unwrap())} else {(0, 0)};
            fkey.cmp(&skey).then(frule.cmp(srule))
        });
        return ordered.join("\n");
    }
    fn expand(&mut self, expression: &String) -> (String, HashSet<String>) {
        let mut rules = HashSet::new();
        let mut result = expression.clone();
        while result.contains("(") {result = self.collapse(&result, &mut rules)};
        let result = self.postfix(&result, &mut rules);
        return (result.trim().to_string(), rules)
    }
    fn collapse(&mut self, expression: &String, rules: &mut HashSet<String>) -> String {
        let Some(hit) = COLLAPSEREGEX.find(expression.as_bytes()) else {return expression.to_string()};
        let inner = &expression[hit.start() + 1 .. hit.end() - 1].to_string();
        let symbol = *self.parentheses.entry(inner.to_string()).or_insert_with(|| {self.counter += 1; self.counter});
        let (expanded, newrules) = self.expand(inner);
        rules.extend(newrules);
        rules.insert(format!("${symbol} -> {expanded}"));
        return format!("{}${symbol}{}", &expression[..hit.start()], &expression[hit.end()..]);
    }
    fn postfix(&mut self, expression: &String, rules: &mut HashSet<String>) -> String {
        let mut result = expression.clone();
        while let Some(hit) = POSTFIXREGEX.captures(&result.as_bytes()) {
            let atom = String::from_utf8(hit.name("atom").unwrap().as_bytes().to_vec()).unwrap();
            let operator = String::from_utf8(hit.name("operator").unwrap().as_bytes().to_vec()).unwrap();
            let symbol = match operator.as_str() {
                "+" => self.more.entry(atom.clone()).or_insert_with(|| {self.counter += 1; self.counter}),
                "*" => self.multiple.entry(atom.clone()).or_insert_with(|| {self.counter += 1; self.counter}),
                "?" => self.optional.entry(atom.clone()).or_insert_with(|| {self.counter += 1; self.counter}),
                other => unreachable!()
            };
            let production = match operator.as_str() {
                "+" => format!("${symbol} -> {atom} ${symbol} | {atom}"),
                "*" => format!("${symbol} -> {atom} ${symbol} |"),
                "?" => format!("${symbol} -> {atom} |"),
                other => unreachable!()
            };
            rules.insert(production);
            let groupzero = hit.get(0).unwrap();
            result = format!("{}${symbol}{}", &result[..groupzero.start()], &result[groupzero.end()..])
        };
        return result;
    }
}


//^
//^ GRAMMAR
//^

//> GRAMMAR -> RULE
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum Rule {
    NonTerminals(NonTerminals),
    Internal(u8)
}

//> GRAMMAR -> SYMBOL
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum Symbol {
    NonTerminals(NonTerminals),
    Internal(u8),
    Kind(Kind)
}

//> GRAMMAR -> NEXUS
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Nexus {
    NonTerminal(NonTerminals),
    Internal(u8),
    Token(Token)
}

//> GRAMMAR -> CONVERT
fn convert(bnf: String) -> HashMap<Rule, Vec<Vec<Symbol>>> {
    let mut syntax: HashMap<Rule, Vec<Vec<Symbol>>> = HashMap::new();
    for line in bnf.lines().map(str::trim) {
        let [rule, productions] = &line.splitn(2, "->").map(|part| part.trim().to_string()).collect::<Vec<String>>()[0..2] else {panic!("{line}")};
        for variant in productions.split("|").map(str::trim) {syntax.entry(match rule {
            internal if internal.starts_with("$") => Rule::Internal(internal[1..].parse().unwrap()),
            symbol => Rule::NonTerminals(NonTerminals::byname(symbol))
        }).or_insert_with(|| Vec::new()).push(if variant.is_empty() {Vec::new()} else {variant.split(" ").map(|atom| match atom {
            internal if internal.starts_with("$") => Symbol::Internal(internal[1..].parse().unwrap()),
            kind if kind == kind.to_uppercase() => Symbol::Kind(Kind::byname(kind)),
            nonterminal => Symbol::NonTerminals(NonTerminals::byname(nonterminal))
        }).collect()})};
    };
    return syntax;
}

//> GRAMMAR -> SYNTAX
pub static SYNTAX: LazyLock<HashMap<Rule, Vec<Vec<Symbol>>>> = LazyLock::new(|| convert(Extensor::new().run("
//> SYNTAX -> START
Start -> (NEWLINES? Level1 SPACES? (NEWLINES Level1 SPACES?)*)? NEWLINES? ENDOFFILE

//> SYNTAX -> 1ºLEVEL
Declaration -> (GROUP SPACES)? Variable SPACES? EQUALITY SPACES? Level2
Definition -> (GROUP SPACES)? Variable SPACES? BINDING SPACES? Level2
Annotation -> GROUP SPACES Variable (SPACES? COMMA SPACES? Variable)*
Node -> Level2
Equation -> Level2 SPACES? EQUALITY SPACES? Level2
Use -> USE SPACES MODULE

//> SYNTAX -> 2ºLEVEL
Expression -> (SIGN SPACES?)* Level3 ((SPACES? SIGN)+ SPACES? Level3)*

//> SYNTAX -> 3ºLEVEL
Term -> Level4 ((SPACES? OPERATOR)? SPACES? Level4)*

//> SYNTAX -> 4ºLEVEL
Factor -> Level5 (EXPONENTIATION SPACES? Level2 SPACES? EXPONENTIATION)?
Limit -> LIMIT SPACES Variable SPACES? TO SPACES? Level2 SIGN? SPACES OF SPACES Nest (EXPONENTIATION SPACES? Level2 SPACES? EXPONENTIATION)?

//> SYNTAX -> 5ºLEVEL
Infinite -> INFINITE
Variable -> IDENTIFIER
Nest -> OPEN SPACES? Level2? SPACES? CLOSE
Tensor -> ENTER SPACES? (Level2 (SPACES? COMMA SPACES? Level2)* SPACES?)? EXIT
Whole -> NUMBER
Absolute -> PIPE SPACES? Level2 SPACES? PIPE
Undefined -> UNDEFINED
Rational -> RATIONAL
Casts -> Level5 GROUP

//> SYNTAX -> LEVELS
Level1 -> Declaration | Definition | Annotation | Node | Equation | Use
Level2 -> Expression
Level3 -> Term
Level4 -> Factor | Limit
Level5 -> Infinite | Variable | Nest | Tensor | Whole | Absolute | Undefined | Rational | Casts
")));