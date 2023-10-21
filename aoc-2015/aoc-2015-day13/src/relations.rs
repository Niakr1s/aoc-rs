use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::From)]
pub struct Happiness(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Idx(usize);

#[derive(Debug)]
pub struct Relation<T, H>
where
    H: Into<Happiness>,
{
    pub from: T,
    pub to: T,
    pub happiness: H,
}

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

    pub fn update_relation<S, H>(&mut self, relation: Relation<S, H>)
    where
        S: AsRef<str>,
        H: Into<Happiness>,
    {
        let Relation {
            from,
            to,
            happiness,
        } = relation;
        let happiness: Happiness = happiness.into();
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
