use std::collections::HashMap;

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

    pub fn update_relation<S>(&mut self, relation: Relation<S>)
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
