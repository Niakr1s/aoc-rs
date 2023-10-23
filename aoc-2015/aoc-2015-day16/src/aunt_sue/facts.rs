use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[derive(Debug, derive_more::Deref)]
pub struct Facts(pub HashMap<String, u32>);

/// Is a match helper for Facts.
pub struct FactsMatcher {
    /// Holds wanted ordinance for facts. Fact not in this will be matched as
    /// equal.
    ord: HashMap<String, Ordering>,
}

impl FactsMatcher {
    pub fn new() -> Self {
        Self {
            ord: HashMap::new(),
        }
    }

    pub fn with_ord(mut self, fact: &str, ord: Ordering) -> Self {
        self.ord.insert(fact.to_owned(), ord);
        self
    }

    /// Returns true if other's keys are in subset of this's keys and they are
    /// eqeual to this's values.
    pub fn is_possible_match(&self, facts: &Facts, other: &Facts) -> bool {
        let self_keys = facts.keys().collect::<HashSet<_>>();
        let other_keys = other.keys().collect::<HashSet<_>>();

        if !other_keys.is_subset(&self_keys) {
            return false;
        }

        other_keys.into_iter().all(|key| {
            let wanted_ord = self.wanted_ord(key);
            let ord = other[key].cmp(&facts[key]);
            wanted_ord == ord
        })
    }

    fn wanted_ord(&self, key: &str) -> Ordering {
        self.ord
            .get(key)
            .map(|&o| Ordering::from(o))
            .unwrap_or(Ordering::Equal)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid input: {0}")]
    InvalidStr(&'static str),
    #[error("ParseIntError")]
    ParseIntError(#[from] std::num::ParseIntError),
}

impl std::str::FromStr for Facts {
    type Err = ParseError;

    /// Example:
    ///
    /// goldfish: 10, trees: 8, perfumes: 6
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(", ");
        let mut facts = HashMap::new();
        while let Some(fact_with_amount) = iter.next() {
            let (fact, amount) =
                fact_with_amount
                    .split_once(": ")
                    .ok_or(ParseError::InvalidStr(
                        "fact and amount should be separated by ': '",
                    ))?;

            facts.insert(fact.to_owned(), amount.parse()?);
        }
        Ok(Facts(facts))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    mod facts_matcher {
        use super::*;

        mod possible_match {
            use super::*;

            mod configured_matcher {
                use super::*;

                #[test]
                fn less() {
                    let facts = Facts(HashMap::from([
                        ("a".to_owned(), 10),
                        ("b".to_owned(), 20),
                        ("c".to_owned(), 30),
                    ]));
                    let matcher = FactsMatcher::new().with_ord("a", Ordering::Less);

                    let valid = Facts(HashMap::from([("a".to_owned(), 5)]));
                    let invalid = Facts(HashMap::from([("a".to_owned(), 15)]));

                    assert_eq!(matcher.is_possible_match(&facts, &valid), true);
                    assert_eq!(matcher.is_possible_match(&facts, &invalid), false);
                }

                #[test]
                fn greater() {
                    let facts = Facts(HashMap::from([
                        ("a".to_owned(), 10),
                        ("b".to_owned(), 20),
                        ("c".to_owned(), 30),
                    ]));
                    let matcher = FactsMatcher::new().with_ord("a", Ordering::Greater);

                    let other = Facts(HashMap::from([("a".to_owned(), 15)]));
                    let invalid = Facts(HashMap::from([("a".to_owned(), 5)]));

                    assert_eq!(matcher.is_possible_match(&facts, &other), true);
                    assert_eq!(matcher.is_possible_match(&facts, &invalid), false);
                }

                #[test]
                fn mixed() {
                    let facts = Facts(HashMap::from([
                        ("a".to_owned(), 10),
                        ("b".to_owned(), 20),
                        ("c".to_owned(), 30),
                    ]));
                    let matcher = FactsMatcher::new()
                        .with_ord("a", Ordering::Greater)
                        .with_ord("b", Ordering::Less);

                    let valid = Facts(HashMap::from([("a".to_owned(), 12), ("b".to_owned(), 18)]));
                    let invalid = Facts(HashMap::from([("a".to_owned(), 8), ("b".to_owned(), 20)]));

                    assert_eq!(matcher.is_possible_match(&facts, &valid), true);
                    assert_eq!(matcher.is_possible_match(&facts, &invalid), false);
                }
            }

            mod default_matcher {
                use super::*;

                #[test]
                fn matches() {
                    let facts = Facts(HashMap::from([
                        ("a".to_owned(), 1),
                        ("b".to_owned(), 2),
                        ("c".to_owned(), 3),
                    ]));
                    let other = Facts(HashMap::from([("a".to_owned(), 1), ("b".to_owned(), 2)]));
                    let matcher = FactsMatcher::new();
                    assert_eq!(matcher.is_possible_match(&facts, &other), true);
                }

                #[test]
                fn with_value_differ() {
                    let facts = Facts(HashMap::from([
                        ("a".to_owned(), 1),
                        ("b".to_owned(), 2),
                        ("c".to_owned(), 3),
                    ]));
                    let other = Facts(HashMap::from([("a".to_owned(), 1), ("b".to_owned(), 3)]));
                    let matcher = FactsMatcher::new();
                    assert_eq!(matcher.is_possible_match(&facts, &other), false);
                }

                #[test]
                fn with_key_differ() {
                    let facts = Facts(HashMap::from([
                        ("a".to_owned(), 1),
                        ("b".to_owned(), 2),
                        ("c".to_owned(), 3),
                    ]));
                    let other = Facts(HashMap::from([("a".to_owned(), 1), ("d".to_owned(), 2)]));
                    let matcher = FactsMatcher::new();
                    assert_eq!(matcher.is_possible_match(&facts, &other), false);
                }
            }
        }
    }

    mod facts {
        use super::*;

        mod from_str {
            use super::*;

            #[test]
            fn works() {
                let input = "cars: 9, akitas: 3, goldfish: 0";
                let facts = input.parse::<Facts>().unwrap();
                assert_eq!(facts.len(), 3);
                assert_eq!(facts["cars"], 9);
                assert_eq!(facts["akitas"], 3);
                assert_eq!(facts["goldfish"], 0);
            }
        }
    }
}
