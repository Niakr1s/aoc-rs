pub mod reindeer;

use reindeer::Reindeer;

pub struct Race;

impl Race {
    pub fn get_distances_after(reindeers: &[Reindeer], time: u32) -> Vec<u32> {
        reindeers
            .iter()
            .map(|r| Race::get_distance_after(r, time))
            .collect()
    }

    pub fn get_distance_after(reindeer: &Reindeer, time: u32) -> u32 {
        let mut state = ReindeerState::Flying(0);
        let mut distance = 0;

        for _elapsed_time in 1..=time {
            state.add_one_sec();

            match state {
                ReindeerState::Flying(flied_time) => {
                    distance += reindeer.speed;
                    if flied_time == reindeer.fly_time {
                        state.switch_state()
                    }
                }
                ReindeerState::Resting(rested_time) => {
                    if rested_time == reindeer.rest_time {
                        state.switch_state()
                    }
                }
            }
        }

        distance
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

    mod race {
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
                assert_eq!(Race::get_distance_after(&reindeer, 1), 14);
                assert_eq!(Race::get_distance_after(&reindeer, 10), 140);
                assert_eq!(Race::get_distance_after(&reindeer, 11), 140);
                assert_eq!(Race::get_distance_after(&reindeer, 12), 140);
                assert_eq!(Race::get_distance_after(&reindeer, 138), 154);
                assert_eq!(Race::get_distance_after(&reindeer, 147), 280);
                assert_eq!(Race::get_distance_after(&reindeer, 1000), 1120);
            }

            #[test]
            fn dancer() {
                let reindeer = Reindeer {
                    name: "Dancer".to_owned(),
                    speed: 16,
                    fly_time: 11,
                    rest_time: 162,
                };
                assert_eq!(Race::get_distance_after(&reindeer, 1), 16);
                assert_eq!(Race::get_distance_after(&reindeer, 10), 160);
                assert_eq!(Race::get_distance_after(&reindeer, 11), 176);
                assert_eq!(Race::get_distance_after(&reindeer, 12), 176);
                assert_eq!(Race::get_distance_after(&reindeer, 174), 192);
                assert_eq!(Race::get_distance_after(&reindeer, 186), 352);
                assert_eq!(Race::get_distance_after(&reindeer, 1000), 1056);
            }
        }
    }
}
