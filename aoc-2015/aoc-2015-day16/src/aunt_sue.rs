pub mod facts;

use self::facts::Facts;

pub struct AuntSue {
    pub no: u32,
    pub facts: Facts,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid input: {0}")]
    InvalidStr(&'static str),
    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Parse facts error: {0}")]
    ParseFactsError(#[from] facts::ParseError),
}

impl std::str::FromStr for AuntSue {
    type Err = ParseError;

    /// Example:
    ///
    /// Sue 25: goldfish: 10, trees: 8, perfumes: 6
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sue, facts) = s.split_once(": ").ok_or(ParseError::InvalidStr(
            "Aunt Sue should be separated from facts by ': '",
        ))?;

        let (_, no) = sue.split_once(" ").ok_or(ParseError::InvalidStr(
            "No should be at the 2nd position and separated from aunt sue by ' '",
        ))?;

        Ok(AuntSue {
            no: no.parse()?,
            facts: facts.parse()?,
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    mod aunt_sue {
        use super::*;

        mod from_str {
            use super::*;

            #[test]
            fn works() {
                let input = "Sue 1: cars: 9, akitas: 3, goldfish: 0";
                let sue = input.parse::<AuntSue>().unwrap();
                assert_eq!(sue.no, 1);
                assert_eq!(sue.facts.len(), 3);
                assert_eq!(sue.facts["cars"], 9);
                assert_eq!(sue.facts["akitas"], 3);
                assert_eq!(sue.facts["goldfish"], 0);
            }
        }
    }
}
