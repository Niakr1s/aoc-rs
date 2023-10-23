pub mod aunt_sue {
    pub mod facts;

    use std::collections::HashMap;

    use self::facts::Facts;

    pub struct AuntSue {
        pub no: u32,
        pub facts: Facts,
    }

    #[derive(Debug, thiserror::Error)]
    pub enum ParseError {
        #[error("Invalid input: {0}")]
        InvalidStr(&'static str),
        #[error("ParseIntError")]
        ParseIntError(#[from] std::num::ParseIntError),
    }

    impl std::str::FromStr for AuntSue {
        type Err = ParseError;

        /// Example:
        ///
        /// Sue 25: goldfish: 10, trees: 8, perfumes: 6
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut iter = s.split_whitespace();

            let no = iter
                .by_ref()
                .skip(1)
                .next()
                .ok_or(ParseError::InvalidStr("len < 2"))?
                .trim_end_matches(':')
                .parse::<u32>()?;

            let mut facts = HashMap::new();

            while let Some(fact) = iter.next() {
                let fact = fact.trim_end_matches(':');
                let amount = iter
                    .next()
                    .ok_or(ParseError::InvalidStr("no value for fact"))?
                    .trim_end_matches(',')
                    .parse::<u32>()?;

                facts.insert(fact.to_owned(), amount);
            }

            Ok(AuntSue {
                no,
                facts: Facts(facts),
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
}
