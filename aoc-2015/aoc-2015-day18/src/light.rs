use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Light {
    On,
    Off,
}

pub struct Grid {
    lights: Vec<Vec<Light>>,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("grid is empty")]
    Empty,
    #[error("grid is not rectangular")]
    LenMismatch,
    #[error("invalid character")]
    InvalidChar,
}

impl FromStr for Grid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lights = s
            .trim()
            .lines()
            .map(|l| {
                l.trim()
                    .chars()
                    .map(|c| -> Result<Light, Self::Err> {
                        match c {
                            '.' => Ok(Light::Off),
                            '#' => Ok(Light::On),
                            _ => Err(ParseError::InvalidChar),
                        }
                    })
                    .collect::<Result<Vec<Light>, Self::Err>>()
            })
            .collect::<Result<Vec<Vec<Light>>, Self::Err>>()?;

        if lights.len() == 0 {
            return Err(ParseError::Empty);
        }

        if lights.len() > 1 {
            let width = lights[0].len();
            if !lights.iter().all(|l| l.len() == width) {
                return Err(ParseError::LenMismatch);
            }
        }

        Ok(Grid { lights })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    mod grid {
        use super::*;

        #[cfg(test)]
        mod from_str {
            use super::*;

            #[test]
            fn works() {
                let input = "#.#.\n#..#\n###.";
                let grid: Grid = input.parse().unwrap();
                assert_eq!(grid.lights.len(), 3);

                assert_eq!(
                    grid.lights[0],
                    vec![Light::On, Light::Off, Light::On, Light::Off]
                );
                assert_eq!(
                    grid.lights[1],
                    vec![Light::On, Light::Off, Light::Off, Light::On]
                );
                assert_eq!(
                    grid.lights[2],
                    vec![Light::On, Light::On, Light::On, Light::Off]
                );
            }
        }
    }
}
