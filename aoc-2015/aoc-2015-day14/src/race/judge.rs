use super::NormalRace;

pub trait Judge {
    fn after_one_sec(&mut self, race: &NormalRace);

    /// Should return None only if it was never called.
    fn scores(&self) -> Option<Vec<u32>>;
}

#[derive(Debug, Clone)]
pub struct LeadingReindeerJudge {
    scores: Option<Vec<u32>>,
}

impl LeadingReindeerJudge {
    pub fn new() -> Self {
        Self { scores: None }
    }

    fn init(&mut self, len: usize) {
        self.scores = Some(vec![0; len]);
    }
}

impl Judge for LeadingReindeerJudge {
    fn after_one_sec(&mut self, race: &NormalRace) {
        if self.scores.is_none() {
            self.init(race.reindeers.len());
        }
        if race.reindeers().is_empty() {
            return;
        }
        let max = *race.distances.iter().max().unwrap();
        let diffs = race.distances.iter().map(|&d| if d == max { 1 } else { 0 });
        self.scores = Some(
            self.scores
                .as_mut()
                .unwrap()
                .iter()
                .zip(diffs)
                .map(|(&s, d)| s + d)
                .collect(),
        );
    }

    fn scores(&self) -> Option<Vec<u32>> {
        self.scores.clone()
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(test)]
    mod leading_reindeer_judge {
        #[allow(unused_imports)]
        use super::*;

        #[cfg(test)]
        mod calculate_scores {
            use crate::reindeer::comet_dancer_vixen;

            use super::*;

            #[test]
            fn empty() {
                let reindeers = vec![];
                let race = NormalRace::new(&reindeers);

                let mut judge = LeadingReindeerJudge::new();
                judge.after_one_sec(&race);
                assert_eq!(judge.scores().unwrap(), vec![]);
            }

            #[test]
            fn one_leader() {
                let reindeers = comet_dancer_vixen();
                let mut race = NormalRace::new(&reindeers);
                race.distances = vec![1, 5, 3];

                let mut judge = LeadingReindeerJudge::new();
                judge.after_one_sec(&race);
                assert_eq!(judge.scores().unwrap(), vec![0, 1, 0]);
            }

            #[test]
            fn several_leaders() {
                let reindeers = comet_dancer_vixen();
                let mut race = NormalRace::new(&reindeers);
                race.distances = vec![3, 1, 3];

                let mut judge = LeadingReindeerJudge::new();
                judge.after_one_sec(&race);
                assert_eq!(judge.scores().unwrap(), vec![1, 0, 1]);
            }
        }
    }
}
