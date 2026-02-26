//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Regex,
    LazyLock,
    Map,
    Set
};

//> HEAD -> LOCAL
use super::grammar::{
    Rule,
    Symbol
};


//^
//^ EXTENSOR
//^

//> EXTENSOR -> PATTERNS
static COLLAPSEREGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\(([^()]+)\)").unwrap());
static POSTFIXREGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?P<atom>\$[0-9]+|[A-Z][a-z]*[0-9]*|_?[A-Z]+)(?P<operator>[*+?])").unwrap());

//> EXTENSOR -> CLASS
pub(super) struct Extensor {
    counter: u8,
    parentheses: Map<String, u8>,
    more: Map<String, u8>,
    multiple: Map<String, u8>,
    optional: Map<String, u8>,
} impl Extensor {
    pub(super) fn run(ebnf: &str) -> Map<Rule, Vec<Vec<Symbol>>> {
        let mut extensor = Self {
            counter: 0,
            parentheses: Map::new(),
            more: Map::new(),
            multiple: Map::new(),
            optional: Map::new(),
        };
        let mut rules = Set::new();
        for line in ebnf.lines().map(str::trim) {
            if line.is_empty() || line.starts_with("//") {continue}
            let (rule, productions) = line.split_once("->").unwrap();
            let body = extensor.expand(productions, &mut rules);
            rules.insert(format!("{rule} -> {body}"));
        };
        rules.insert("$0 -> Start".to_string());
        let mut ordered = rules.into_iter().collect::<Vec<String>>();
        ordered.sort_by(|first, second| {
            let [frule, srule] = [first, second].map(|each| each.splitn(2, "->").next().unwrap().trim());
            let [fkey, skey] = [frule, srule].map(|each| if each.starts_with('$') {(1, each[1..].parse::<usize>().unwrap())} else {(0, 0)});
            fkey.cmp(&skey).then(frule.cmp(srule))
        });
        return extensor.serialize(ordered.join("\n"));
    }
    fn expand(&mut self, expression: &str, rules: &mut Set<String>) -> String {
        let mut result = expression.to_string();
        while result.contains("(") {result = self.collapse(&result, rules)};
        result = self.postfix(&result, rules);
        return result.to_string();
    }
    fn collapse(&mut self, expression: &str, rules: &mut Set<String>) -> String {
        let Some(hit) = COLLAPSEREGEX.find(expression.as_bytes()) else {return expression.to_string()};
        let inner = &expression[hit.start() + 1 .. hit.end() - 1].to_string();
        let symbol = *self.parentheses.entry(inner.to_string()).or_insert_with(|| {self.counter += 1; self.counter});
        let expanded = self.expand(inner, rules);
        rules.insert(format!("${symbol} -> {expanded}"));
        return format!("{}${symbol}{}", &expression[..hit.start()], &expression[hit.end()..]);
    }
    fn postfix(&mut self, expression: &str, rules: &mut Set<String>) -> String {
        let mut result = expression.to_string();
        while let Some(hit) = POSTFIXREGEX.captures(&result.as_bytes()) {
            let atom = String::from_utf8(hit.name("atom").unwrap().as_bytes().to_vec()).unwrap();
            let operator = String::from_utf8(hit.name("operator").unwrap().as_bytes().to_vec()).unwrap();
            let symbol = match &operator as &str {
                "+" => self.more.entry(atom.clone()).or_insert_with(|| {self.counter += 1; self.counter}),
                "*" => self.multiple.entry(atom.clone()).or_insert_with(|| {self.counter += 1; self.counter}),
                "?" => self.optional.entry(atom.clone()).or_insert_with(|| {self.counter += 1; self.counter}),
                other => unreachable!()
            };
            rules.insert(match &operator as &str {
                "+" => format!("${symbol} -> {atom} ${symbol} | {atom}"),
                "*" => format!("${symbol} -> {atom} ${symbol} |"),
                "?" => format!("${symbol} -> {atom} |"),
                other => unreachable!()
            });
            let groupzero = hit.get(0).unwrap();
            result = format!("{}${symbol}{}", &result[..groupzero.start()], &result[groupzero.end()..])
        };
        return result;
    }
    fn serialize(&self, bnf: String) -> Map<Rule, Vec<Vec<Symbol>>> {
        let mut map = Map::new();
        for line in bnf.lines() {
            let [rule, productions] = line.splitn(2, "->").map(str::trim).collect::<Vec<&str>>()[0..2] else {panic!("{line}")};
            for variant in productions.split("|").map(str::trim) {
                map.entry(rule.into()).or_insert_with(Vec::new).push(if variant.is_empty() {
                    Vec::new()
                } else {
                    variant.split(" ").map(Symbol::from).collect()
                });
            }
        }
        return map;
    }
}