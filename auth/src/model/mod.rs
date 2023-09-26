use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Scope(HashSet<String>);

#[derive(Debug)]
enum Pattern {
    Equal(String),
    Prefix(String),
    Regex(regex::Regex),
}

impl Pattern {
    fn matched(&self, url: String) -> bool {
        match self {
            Pattern::Equal(value) => *value == url,
            Pattern::Prefix(value) => url.starts_with(value),
            Pattern::Regex(value) => value.is_match(url.as_str()),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pattern: Pattern,
    scope: Scope,
}

impl Config {
    fn get_scope(&self, url: String) -> Option<Scope> {
        let matched = self.pattern.matched(url);
        if matched {
            Some(self.scope.to_owned())
        } else {
            None
        }
    }
}
