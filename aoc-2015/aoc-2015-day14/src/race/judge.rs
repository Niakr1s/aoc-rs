use super::NormalRace;

pub trait Judge {
    fn calculate_scores(&self, race: &NormalRace) -> Vec<u32>;
}

#[derive(Debug, Clone)]
pub struct LeadingReindeerJudge;

impl Judge for LeadingReindeerJudge {
    fn calculate_scores(&self, race: &NormalRace) -> Vec<u32> {
        if race.reindeers.is_empty() {
            return vec![];
        }
        let max = *race.distances.iter().max().unwrap();
        race.distances
            .iter()
            .map(|&d| if d == max { 1 } else { 0 })
            .collect()
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

                let judge = LeadingReindeerJudge;
                let scores = judge.calculate_scores(&race);
                assert_eq!(scores, vec![]);
            }

            #[test]
            fn one_leader() {
                let reindeers = comet_dancer_vixen();
                let mut race = NormalRace::new(&reindeers);
                race.distances = vec![1, 5, 3];

                let judge = LeadingReindeerJudge;
                let scores = judge.calculate_scores(&race);
                assert_eq!(scores, vec![0, 1, 0]);
            }

            #[test]
            fn several_leaders() {
                let reindeers = comet_dancer_vixen();
                let mut race = NormalRace::new(&reindeers);
                race.distances = vec![3, 1, 3];

                let judge = LeadingReindeerJudge;
                let scores = judge.calculate_scores(&race);
                assert_eq!(scores, vec![1, 0, 1]);
            }
        }
    }
}
