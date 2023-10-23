use std::collections::{HashMap, HashSet};

#[derive(Debug, derive_more::Deref)]
pub struct Facts(pub HashMap<String, u32>);

impl Facts {
    /// Returns true if other's keys are in subset of this's keys and they are
    /// eqeual to this's values.
    pub fn possible_eq(&self, other: &Self) -> bool {
        let self_keys = self.keys().collect::<HashSet<_>>();
        let other_keys = other.keys().collect::<HashSet<_>>();

        if !other_keys.is_subset(&self_keys) {
            return false;
        }

        other_keys.into_iter().all(|key| self[key] == other[key])
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

        mod possible_eq {
            use super::*;

            #[test]
            fn eq() {
                let facts = Facts(HashMap::from([
                    ("a".to_owned(), 1),
                    ("b".to_owned(), 2),
                    ("c".to_owned(), 3),
                ]));
                let other = Facts(HashMap::from([("a".to_owned(), 1), ("b".to_owned(), 2)]));
                assert_eq!(facts.possible_eq(&other), true);
            }

            #[test]
            fn with_value_differ() {
                let facts = Facts(HashMap::from([
                    ("a".to_owned(), 1),
                    ("b".to_owned(), 2),
                    ("c".to_owned(), 3),
                ]));
                let other = Facts(HashMap::from([("a".to_owned(), 1), ("b".to_owned(), 3)]));
                assert_eq!(facts.possible_eq(&other), false);
            }

            #[test]
            fn with_key_differ() {
                let facts = Facts(HashMap::from([
                    ("a".to_owned(), 1),
                    ("b".to_owned(), 2),
                    ("c".to_owned(), 3),
                ]));
                let other = Facts(HashMap::from([("a".to_owned(), 1), ("d".to_owned(), 2)]));
                assert_eq!(facts.possible_eq(&other), false);
            }
        }
    }
}
