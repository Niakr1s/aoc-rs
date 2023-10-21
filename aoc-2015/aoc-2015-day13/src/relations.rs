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
pub struct RelationMap {
    next_idx: Idx,
    participants: HashMap<String, Idx>,
    relations: HashMap<Idx, HashMap<Idx, Happiness>>,
}

impl RelationMap {
    pub fn new() -> Self {
        RelationMap {
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

    pub fn calculate_happiness(&self, table: &[Idx]) -> Option<Happiness> {
        if !self.is_correct() {
            return None;
        }

        let has_unknown_participant = HashSet::<&Idx>::from_iter(table.iter())
            .difference(&HashSet::<&Idx>::from_iter(self.participants.values()))
            .count()
            != 0;
        if has_unknown_participant {
            return None;
        }

        match table.len() {
            0 | 1 => Some(0.into()),
            2 => Some(
                (*self.relations[&table[0]][&table[1]] + *self.relations[&table[1]][&table[0]])
                    .into(),
            ),
            len @ _ => {
                let mut happiness = 0;
                for i in 0..len {
                    let subject = table[i];
                    let (left, right) = if i == 0 {
                        (table[len - 1], table[i + 1])
                    } else if i == len - 1 {
                        (table[i - 1], table[0])
                    } else {
                        (table[i - 1], table[i + 1])
                    };
                    happiness +=
                        *self.relations[&subject][&left] + *self.relations[&subject][&right];
                }
                Some(happiness.into())
            }
        }
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

pub mod relation_map {
    use super::*;

    #[cfg(test)]
    mod tests {
        use super::*;

        mod calculate_happiness {
            use super::*;

            #[test]
            fn unknown_participants() {
                let relation_map = RelationMap::new();
                assert_eq!(relation_map.calculate_happiness(&[Idx(0)]), None);
            }

            #[test]
            fn zero_participants() {
                let relation_map = RelationMap::new();
                assert_eq!(relation_map.calculate_happiness(&[]), Some(0.into()));
            }

            #[test]
            fn two_participants() {
                let mut relation_map = RelationMap::new();
                relation_map.update_relation(rel!("Alice", "Bob", 54));
                let (bob, alice) = relation_map.update_relation(rel!("Bob", "Alice", -33));
                assert_eq!(
                    relation_map.calculate_happiness(&[alice, bob]),
                    Some(21.into())
                );
                assert_eq!(relation_map.calculate_happiness(&[alice]), Some(0.into()));
                assert_eq!(relation_map.calculate_happiness(&[bob]), Some(0.into()));
                assert_eq!(
                    relation_map.calculate_happiness(&[alice, bob, Idx(333)]),
                    None
                );
            }

            #[test]
            fn three_participants() {
                let mut relation_map = RelationMap::new();
                let (alice, bob) = relation_map.update_relation(rel!("Alice", "Bob", 54));
                relation_map.update_relation(rel!("Bob", "Alice", -33));
                relation_map.update_relation(rel!("Alice", "Fred", 123));
                relation_map.update_relation(rel!("Bob", "Fred", 532));
                relation_map.update_relation(rel!("Fred", "Alice", -333));
                let (fred, _) = relation_map.update_relation(rel!("Fred", "Bob", -222));
                assert_eq!(relation_map.calculate_happiness(&[alice]), Some(0.into()));
                assert_eq!(relation_map.calculate_happiness(&[bob]), Some(0.into()));
                assert_eq!(
                    relation_map.calculate_happiness(&[alice, bob, Idx(333)]),
                    None
                );
                assert_eq!(
                    relation_map.calculate_happiness(&[alice, bob, fred]),
                    Some(121.into())
                );
            }

            #[test]
            fn many_participants() {
                const PARTICIPANTS: &'static str = r#"Alice would gain 54 happiness units by sitting next to Bob.
                Alice would lose 79 happiness units by sitting next to Carol.
                Alice would lose 2 happiness units by sitting next to David.
                Bob would gain 83 happiness units by sitting next to Alice.
                Bob would lose 7 happiness units by sitting next to Carol.
                Bob would lose 63 happiness units by sitting next to David.
                Carol would lose 62 happiness units by sitting next to Alice.
                Carol would gain 60 happiness units by sitting next to Bob.
                Carol would gain 55 happiness units by sitting next to David.
                David would gain 46 happiness units by sitting next to Alice.
                David would lose 7 happiness units by sitting next to Bob.
                David would gain 41 happiness units by sitting next to Carol."#;

                let mut relation_map = RelationMap::new();
                for line in PARTICIPANTS.lines() {
                    relation_map.update_relation(Relation::from_adventofcode_line(line).unwrap());
                }

                let table = &[
                    relation_map.participants["Alice"],
                    relation_map.participants["Bob"],
                    relation_map.participants["Carol"],
                    relation_map.participants["David"],
                ];

                assert_eq!(relation_map.calculate_happiness(table), Some(330.into()));
            }
        }

        mod is_correct {
            use super::*;

            #[test]
            fn zero_participants() {
                let relation_map = RelationMap::new();
                assert_eq!(relation_map.is_correct(), true);
            }

            #[test]
            fn one_participant() {
                // It's contrived example with accessing to private members, but let it be.
                let mut relation_map = RelationMap::new();
                relation_map.participants.insert("Bob".to_owned(), Idx(0));
                relation_map.next_idx = Idx(1);
                assert_eq!(relation_map.is_correct(), false);
            }

            #[test]
            fn two_participants() {
                let mut relation_map = RelationMap::new();
                relation_map.update_relation(rel!("Alice", "Bob", 54));
                assert_eq!(relation_map.is_correct(), false);
                relation_map.update_relation(rel!("Bob", "Alice", -33));
                assert_eq!(relation_map.is_correct(), true);
            }

            #[test]
            fn three_participants() {
                let mut relation_map = RelationMap::new();
                relation_map.update_relation(rel!("Alice", "Bob", 54));
                assert_eq!(relation_map.is_correct(), false);
                relation_map.update_relation(rel!("Bob", "Alice", -33));
                assert_eq!(relation_map.is_correct(), true);
                relation_map.update_relation(rel!("Alice", "Fred", 123));
                assert_eq!(relation_map.is_correct(), false);
                relation_map.update_relation(rel!("Bob", "Fred", 123));
                assert_eq!(relation_map.is_correct(), false);
                relation_map.update_relation(rel!("Fred", "Alice", -333));
                assert_eq!(relation_map.is_correct(), false);
                relation_map.update_relation(rel!("Fred", "Bob", -333));
                assert_eq!(relation_map.is_correct(), true);
            }
        }

        mod update_relation {
            use super::*;

            #[test]
            fn correctly_updates_relation() {
                let mut relation_map = RelationMap::new();

                let (from, to) = relation_map.update_relation(rel!("Alice", "Bob", 54));
                assert_ne!(from, to);
                assert_eq!(relation_map.participants["Alice"], from);
                assert_eq!(relation_map.participants["Bob"], to);
                assert_eq!(*relation_map.relations[&to][&from], 54);
                assert_eq!(relation_map.relations.contains_key(&from), false);

                let (from, to) = relation_map.update_relation(rel!("Bob", "Alice", -33));
                assert_ne!(from, to);
                assert_eq!(*relation_map.relations[&to][&from], -33);
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
