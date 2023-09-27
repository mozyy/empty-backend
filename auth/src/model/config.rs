use std::{cmp, collections::HashSet, fmt, str::FromStr};

use empty_utils::errors::{Error, ErrorConvert};
use proto::pb;

#[derive(Clone, PartialEq, Default)]
pub struct Scope(HashSet<String>);

impl Scope {
    pub fn is_empty(&self) -> bool {
        return self.0.is_empty();
    }
}

impl cmp::PartialOrd for Scope {
    fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
        let intersect_count = self.0.intersection(&rhs.0).count();
        if intersect_count == self.0.len() && intersect_count == rhs.0.len() {
            Some(cmp::Ordering::Equal)
        } else if intersect_count == self.0.len() {
            Some(cmp::Ordering::Less)
        } else if intersect_count == rhs.0.len() {
            Some(cmp::Ordering::Greater)
        } else {
            None
        }
    }
}

impl fmt::Debug for Scope {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("Scope").field(&self.0).finish()
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let output = self
            .0
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(" ");
        fmt.write_str(&output)
    }
}

impl FromStr for Scope {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(' ').filter(|s| !s.is_empty());
        Ok(Scope(tokens.map(str::to_string).collect()))
    }
}

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

impl TryFrom<pb::auth::auth::Pattern> for Pattern {
    type Error = Error;
    fn try_from(value: pb::auth::auth::Pattern) -> Result<Self, Self::Error> {
        let pattern = value.pattern.ok_or_loss()?;
        let pattern = match pattern {
            pb::auth::auth::pattern::Pattern::Equal(value) => Pattern::Equal(value),
            pb::auth::auth::pattern::Pattern::Prefix(value) => Pattern::Prefix(value),
            pb::auth::auth::pattern::Pattern::Regex(value) => {
                Pattern::Regex(value.parse().ok_or_invalid()?)
            }
        };
        Ok(pattern)
    }
}

#[derive(Debug)]
pub struct Config {
    pattern: Pattern,
    scope: Scope,
}

impl Config {
    pub fn get_scope(&self, uri: String) -> Option<Scope> {
        let matched = self.pattern.matched(uri);
        if matched {
            Some(self.scope.to_owned())
        } else {
            None
        }
    }
}

impl TryFrom<pb::auth::auth::Config> for Config {
    type Error = Error;
    fn try_from(value: pb::auth::auth::Config) -> Result<Self, Self::Error> {
        let scope = value.scope.unwrap_or_default().parse()?;
        let pattern = value.pattern.ok_or_loss()?.try_into()?;
        Ok(Self { scope, pattern })
    }
}
