pub mod judge;

use self::judge::Judge;
use crate::reindeer::Reindeer;

pub trait Race: Clone {
    fn after(self, secs: u32) -> Self;
    fn scores(&self) -> Vec<u32>;
    fn reindeers(&self) -> &[&Reindeer];
    fn distances(&self) -> &[u32];
}

#[derive(Debug, Clone)]
pub struct NormalRace<'a> {
    elapsed: u32,
    reindeers: Vec<&'a Reindeer>,
    states: Vec<ReindeerState>,
    distances: Vec<u32>,
}

impl<'a> NormalRace<'a> {
    pub fn new(reindeers: &'a [Reindeer]) -> NormalRace<'a> {
        NormalRace {
            elapsed: 0,
            reindeers: reindeers.iter().collect(),
            states: vec![ReindeerState::Flying(0); reindeers.len()],
            distances: vec![0; reindeers.len()],
        }
    }

    fn after_one_sec(&mut self) {
        self.elapsed += 1;

        for (i, &reindeer) in self.reindeers.iter().enumerate() {
            self.states[i].add_one_sec();

            match self.states[i] {
                ReindeerState::Flying(flied_time) => {
                    self.distances[i] += reindeer.speed;
                    if flied_time == reindeer.fly_time {
                        self.states[i].switch_state()
                    }
                }
                ReindeerState::Resting(rested_time) => {
                    if rested_time == reindeer.rest_time {
                        self.states[i].switch_state()
                    }
                }
            }
        }
    }

    pub fn distances(&self) -> &[u32] {
        self.distances.as_ref()
    }

    pub fn reindeers(&self) -> &[&Reindeer] {
        self.reindeers.as_ref()
    }
}

impl<'a> Race for NormalRace<'a> {
    fn after(mut self, secs: u32) -> Self {
        (0..secs).for_each(|_| self.after_one_sec());
        self
    }

    fn scores(&self) -> Vec<u32> {
        self.distances.to_vec()
    }

    fn reindeers(&self) -> &[&Reindeer] {
        self.reindeers.as_ref()
    }

    fn distances(&self) -> &[u32] {
        self.distances.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct JudgedRace<'a, J> {
    race: NormalRace<'a>,
    judge: J,
}

impl<'a, J> JudgedRace<'a, J>
where
    J: Judge + Clone,
{
    pub fn new(reindeers: &'a [Reindeer], judge: J) -> JudgedRace<'a, J> {
        let race = NormalRace::new(reindeers);
        JudgedRace { race, judge }
    }
}

impl<'a, J> Race for JudgedRace<'a, J>
where
    J: Judge + Clone,
{
    fn after(mut self, secs: u32) -> Self {
        for _ in 0..secs {
            self.race.after_one_sec();
            self.judge.after_one_sec(&self.race);
        }
        self
    }

    fn scores(&self) -> Vec<u32> {
        self.judge
            .scores()
            .unwrap_or(vec![0; self.reindeers().len()])
    }

    fn reindeers(&self) -> &[&Reindeer] {
        self.race.reindeers()
    }

    fn distances(&self) -> &[u32] {
        self.race.distances()
    }
}

#[derive(Debug, Clone, Copy)]
enum ReindeerState {
    Flying(u32),
    Resting(u32),
}

impl ReindeerState {
    fn add_one_sec(&mut self) {
        *self = match self {
            ReindeerState::Flying(secs) => ReindeerState::Flying(*secs + 1),
            ReindeerState::Resting(secs) => ReindeerState::Resting(*secs + 1),
        }
    }

    fn switch_state(&mut self) {
        *self = match self {
            ReindeerState::Flying(_) => ReindeerState::Resting(0),
            ReindeerState::Resting(_) => ReindeerState::Flying(0),
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    mod judged_race {
        use crate::{race::judge::LeadingReindeerJudge, reindeer::comet_dancer_vixen};

        use super::*;

        #[test]
        /// Given the example reindeer from above, after the first second,
        /// Dancer is in the lead and gets one point. He stays in the lead until
        /// several seconds into Comet's second burst: after the 140th second,
        /// Comet pulls into the lead and gets his first point. Of course, since
        /// Dancer had been in the lead for the 139 seconds before that, he has
        /// accumulated 139 points by the 140th second.
        ///
        /// After the 1000th second, Dancer has accumulated 689 points, while
        /// poor Comet, our old champion, only has 312. So, with the new scoring
        /// system, Dancer would win (if the race ended at 1000 seconds).
        fn judge_works() {
            let reindeers = comet_dancer_vixen();
            let judged_race = JudgedRace::new(&reindeers[0..2], LeadingReindeerJudge::new());

            assert_eq!(judged_race.clone().after(1).scores(), vec![0, 1]);
            assert_eq!(judged_race.clone().after(140).scores(), vec![1, 139]);
            assert_eq!(judged_race.clone().after(1000).scores(), vec![312, 689]);
        }
    }

    mod normal_race {
        use super::*;

        mod get_distance_after {
            use crate::reindeer::comet_dancer_vixen;

            use super::*;

            // After one second, Comet has gone 14 km, while Dancer has gone 16
            // km. After ten seconds, Comet has gone 140 km, while Dancer has
            // gone 160 km. On the eleventh second, Comet begins resting
            // (staying at 140 km), and Dancer continues on for a total
            // distance of 176 km. On the 12th second, both reindeer are
            // resting. They continue to rest until the 138th second, when
            // Comet flies for another ten seconds. On the 174th second, Dancer
            // flies for another 11 seconds.

            // In this example, after the 1000th second, both reindeer are
            // resting, and Comet is in the lead at 1120 km (poor Dancer has
            // only gotten 1056 km by that point). So, in this situation, Comet
            // would win (if the race ended at 1000 seconds).

            #[test]
            fn comet() {
                let reindeers = comet_dancer_vixen();
                let race = NormalRace::new(&reindeers);
                assert_eq!(race.clone().after(1).distances[0], 14);
                assert_eq!(race.clone().after(10).distances[0], 140);
                assert_eq!(race.clone().after(11).distances[0], 140);
                assert_eq!(race.clone().after(12).distances[0], 140);
                assert_eq!(race.clone().after(138).distances[0], 154);
                assert_eq!(race.clone().after(147).distances[0], 280);
                assert_eq!(race.clone().after(1000).distances[0], 1120);
            }

            #[test]
            fn dancer() {
                let reindeers = comet_dancer_vixen();
                let race = NormalRace::new(&reindeers);
                assert_eq!(race.clone().after(1).distances[1], 16);
                assert_eq!(race.clone().after(10).distances[1], 160);
                assert_eq!(race.clone().after(11).distances[1], 176);
                assert_eq!(race.clone().after(12).distances[1], 176);
                assert_eq!(race.clone().after(174).distances[1], 192);
                assert_eq!(race.clone().after(186).distances[1], 352);
                assert_eq!(race.clone().after(1000).distances[1], 1056);
            }
        }
    }
}
