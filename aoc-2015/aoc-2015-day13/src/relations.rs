use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::From, derive_more::Deref,
)]
pub struct Happiness(i32);

#[derive(Debug)]
pub struct Relation<T> {
    pub from: T,
    pub to: T,
    pub happiness: Happiness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Idx(usize);

#[derive(Debug)]
pub struct Relations {
    next_idx: Idx,
    participants: HashMap<String, Idx>,
    relations: HashMap<Idx, HashMap<Idx, Happiness>>,
}

impl Relations {
    pub fn new() -> Self {
        Relations {
            next_idx: Idx(0),
            participants: HashMap::new(),
            relations: HashMap::new(),
        }
    }

    /// Returns (from, to) pair
    pub fn update_relation<S>(&mut self, relation: Relation<S>) -> (Idx, Idx)
    where
        S: AsRef<str>,
    {
        let Relation {
            from,
            to,
            happiness,
        } = relation;
        let from = self.try_add_participant(from.as_ref());
        let to = self.try_add_participant(to.as_ref());

        self.relations
            .entry(to)
            .or_default()
            .insert(from, happiness);
        (from, to)
    }

    /// Each of n participants should have n-1 relations to others.
    /// Zero participants also correct.
    /// One participant should be incorrect, because we always add relation of
    /// minimum 2 participants.
    pub fn is_correct(&self) -> bool {
        match self.participants.len() {
            0 => self.relations.len() == 0,
            1 => false,
            _ => {
                let (r, p) = (
                    HashSet::<&Idx>::from_iter(self.relations.keys()),
                    HashSet::<&Idx>::from_iter(self.participants.values()),
                );
                println!("{:?} {:?}", r, p);
                if p.difference(&r).count() != 0 {
                    return false;
                }
                self.relations.iter().all(|(to, from)| {
                    let froms = HashSet::<&Idx>::from_iter(from.keys());
                    let mut diff = p.difference(&froms);
                    let has_to = diff.next() == Some(&&to);
                    has_to && diff.next().is_none()
                })
            }
        }
    }

    /// Adds participant if not exists
    fn try_add_participant(&mut self, participant: &str) -> Idx {
        if !self.participants.contains_key(participant) {
            self.participants
                .insert(participant.to_owned(), self.next_idx);
            self.incr_next_idx();
        }
        self.participants[participant]
    }

    fn incr_next_idx(&mut self) {
        self.next_idx = Idx(self.next_idx.0 + 1);
    }
}

macro_rules! rel {
    ($from:expr, $to:expr, $happiness:expr) => {
        Relation {
            from: $from,
            to: $to,
            happiness: Happiness($happiness),
        }
    };
}

pub mod relations {
    use super::*;

    #[cfg(test)]
    mod tests {
        use super::*;

        mod is_correct {
            use super::*;

            #[test]
            fn zero_participants() {
                let relations = Relations::new();
                assert_eq!(relations.is_correct(), true);
            }

            #[test]
            fn one_participant() {
                // It's contrived example with accessing to private members, but let it be.
                let mut relations = Relations::new();
                relations.participants.insert("Bob".to_owned(), Idx(0));
                relations.next_idx = Idx(1);
                assert_eq!(relations.is_correct(), false);
            }

            #[test]
            fn two_participants() {
                let mut relations = Relations::new();
                relations.update_relation(rel!("Alice", "Bob", 54));
                assert_eq!(relations.is_correct(), false);
                relations.update_relation(rel!("Bob", "Alice", -33));
                assert_eq!(relations.is_correct(), true);
            }

            #[test]
            fn three_participants() {
                let mut relations = Relations::new();
                relations.update_relation(rel!("Alice", "Bob", 54));
                assert_eq!(relations.is_correct(), false);
                relations.update_relation(rel!("Bob", "Alice", -33));
                assert_eq!(relations.is_correct(), true);
                relations.update_relation(rel!("Alice", "Fred", 123));
                assert_eq!(relations.is_correct(), false);
                relations.update_relation(rel!("Bob", "Fred", 123));
                assert_eq!(relations.is_correct(), false);
                relations.update_relation(rel!("Fred", "Alice", -333));
                assert_eq!(relations.is_correct(), false);
                relations.update_relation(rel!("Fred", "Bob", -333));
                assert_eq!(relations.is_correct(), true);
            }
        }

        mod update_relation {
            use super::*;

            #[test]
            fn correctly_updates_relation() {
                let mut relations = Relations::new();

                let (from, to) = relations.update_relation(rel!("Alice", "Bob", 54));
                assert_ne!(from, to);
                assert_eq!(relations.participants["Alice"], from);
                assert_eq!(relations.participants["Bob"], to);
                assert_eq!(*relations.relations[&to][&from], 54);
                assert_eq!(relations.relations.contains_key(&from), false);

                let (from, to) = relations.update_relation(rel!("Bob", "Alice", -33));
                assert_ne!(from, to);
                assert_eq!(*relations.relations[&to][&from], -33);
            }
        }
    }
}

pub mod relation {
    use itertools::Itertools;

    use super::*;

    #[derive(Debug, thiserror::Error)]
    pub enum IntoRelationError {
        #[error("invalid line length")]
        InvalidLineLength,
        #[error("invalid happiness sign")]
        InvalidHappinessSign,
        #[error("invalid happiness value")]
        InvalidHappinessValue,
    }

    impl Relation<&str> {
        /// Example line:
        ///
        /// Alice would gain 54 happiness units by sitting next to Bob.
        pub fn from_adventofcode_line(mut line: &str) -> Result<Relation<&str>, IntoRelationError> {
            if line.ends_with(".") {
                line = &line[..line.len() - 1];
            }

            let words = line.split_whitespace().collect_vec();

            let (to, happiness_sign, happiness, from) = (0, 2, 3, 10);
            if words.len() != 11 {
                return Err(IntoRelationError::InvalidLineLength);
            } else {
                let happiness_sign = match words[happiness_sign] {
                    "gain" => 1,
                    "lose" => -1,
                    _ => return Err(IntoRelationError::InvalidHappinessSign),
                };
                let happiness: Happiness = words[happiness]
                    .parse::<i32>()
                    .map(|x| x * happiness_sign)
                    .map_err(|_| IntoRelationError::InvalidHappinessValue)?
                    .into();
                Ok(Relation {
                    from: words[from],
                    to: words[to],
                    happiness,
                })
            }
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        mod from_adventofcode_line {
            use super::*;

            #[test]
            fn alice_from_bob_plus_happiness() {
                let line = "Alice would gain 54 happiness units by sitting next to Bob.";
                let relation = Relation::from_adventofcode_line(line).unwrap();
                assert_eq!(relation.to, "Alice");
                assert_eq!(relation.from, "Bob");
                assert_eq!(*relation.happiness, 54);
            }

            #[test]
            fn alice_from_bob_minus_happiness() {
                let line = "Alice would lose 54 happiness units by sitting next to Bob.";
                let relation = Relation::from_adventofcode_line(line).unwrap();
                assert_eq!(relation.to, "Alice");
                assert_eq!(relation.from, "Bob");
                assert_eq!(*relation.happiness, -54);
            }
        }
    }
}
