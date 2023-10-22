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
                let reindeer = Reindeer::from_aoc_line(line).unwrap();
                assert_eq!(reindeer.name, "Vixen");
                assert_eq!(reindeer.speed, 8);
                assert_eq!(reindeer.fly_time, 3);
                assert_eq!(reindeer.rest_time, 53);
            }
        }
    }
}
