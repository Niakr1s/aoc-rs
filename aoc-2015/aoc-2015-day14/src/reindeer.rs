#[derive(Debug, Clone)]
pub struct Reindeer {
    pub name: String,
    pub speed: u32,
    pub fly_time: u32,
    pub rest_time: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum FromAocLineIntoReindeerError {
    #[error("Lengh of words doesn't match")]
    LenMismatch,
    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

impl Reindeer {
    /// Example:
    ///
    /// Vixen can fly 8 km/s for 3 seconds, but then must rest for 53 seconds.
    pub fn from_aoc_line(line: &str) -> Result<Reindeer, FromAocLineIntoReindeerError> {
        let expected_words_len = 15;
        let (name, speed, fly_time, rest_time) = (0, 3, 6, 13);

        let words = line.split_whitespace().collect::<Vec<_>>();
        if words.len() != expected_words_len {
            return Err(FromAocLineIntoReindeerError::LenMismatch);
        }

        let (name, speed, fly_time, rest_time) = (
            words[name].to_owned(),
            words[speed].parse()?,
            words[fly_time].parse()?,
            words[rest_time].parse()?,
        );

        Ok(Reindeer {
            name,
            speed,
            fly_time,
            rest_time,
        })
    }
}

#[cfg(test)]
/// Create 3 reindeers: Comet, Dancer, Vixen:
///
/// Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
///
/// Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
///
/// Vixen can fly 8 km/s for 3 seconds, but then must rest for 53 seconds.
pub fn comet_dancer_vixen() -> Vec<Reindeer> {
    vec![
        Reindeer {
            name: "Comet".to_owned(),
            speed: 14,
            fly_time: 10,
            rest_time: 127,
        },
        Reindeer {
            name: "Dancer".to_owned(),
            speed: 16,
            fly_time: 11,
            rest_time: 162,
        },
        Reindeer {
            name: "Vixen".to_owned(),
            speed: 8,
            fly_time: 3,
            rest_time: 53,
        },
    ]
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    mod reindeer {
        #[allow(unused_imports)]
        use super::*;

        mod from_aoc_line {
            #[allow(unused_imports)]
            use super::*;

            #[test]
            fn valid_line() {
                let line = "Vixen can fly 8 km/s for 3 seconds, but then must rest for 53 seconds.";
                let parsed = Reindeer::from_aoc_line(line).unwrap();
                let reindeers = comet_dancer_vixen();
                let wanted = &reindeers[2];
                assert_eq!(parsed.name, wanted.name);
                assert_eq!(parsed.speed, wanted.speed);
                assert_eq!(parsed.fly_time, wanted.fly_time);
                assert_eq!(parsed.rest_time, wanted.rest_time);
            }
        }
    }
}
