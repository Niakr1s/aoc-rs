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

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    mod facts {
        use super::*;

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
