use crate::reindeer::Reindeer;

pub trait Race {
    fn after(self, secs: u32) -> Self;
    fn scores(&self) -> Vec<u32>;
}

#[derive(Debug, Clone)]
pub struct NormalRace<'a> {
    elapsed: u32,
    reindeers: Vec<&'a Reindeer>,
    states: Vec<ReindeerState>,
    distances: Vec<u32>,
}

pub struct Score<'a> {
    pub reindeer: &'a Reindeer,
    pub score: u32,
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
}

impl<'a> Race for NormalRace<'a> {
    fn after(mut self, secs: u32) -> Self {
        (0..secs).for_each(|_| self.after_one_sec());
        self
    }

    fn scores(&self) -> Vec<u32> {
        self.distances.to_vec()
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
mod tests {
    #[allow(unused_imports)]
    use super::*;

    mod get_distance_after {
        use super::*;

        // For example, suppose you have the following Reindeer:
        //
        // Comet can fly 14 km/s for 10 seconds, but then must rest for 127
        // seconds.
        //
        // Dancer can fly 16 km/s for 11 seconds, but then must
        // rest for 162 seconds.
        //
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
            let reindeer = Reindeer {
                name: "Comet".to_owned(),
                speed: 14,
                fly_time: 10,
                rest_time: 127,
            };
            let binding = [reindeer];
            let race = NormalRace::new(&binding);
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
            let reindeer = Reindeer {
                name: "Dancer".to_owned(),
                speed: 16,
                fly_time: 11,
                rest_time: 162,
            };
            let binding = [reindeer];
            let race = NormalRace::new(&binding);
            assert_eq!(race.clone().after(1).distances[0], 16);
            assert_eq!(race.clone().after(10).distances[0], 160);
            assert_eq!(race.clone().after(11).distances[0], 176);
            assert_eq!(race.clone().after(12).distances[0], 176);
            assert_eq!(race.clone().after(174).distances[0], 192);
            assert_eq!(race.clone().after(186).distances[0], 352);
            assert_eq!(race.clone().after(1000).distances[0], 1056);
        }
    }
}