use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ingredient {
    pub name: String,
    pub capacity: i32,
    pub durability: i32,
    pub flavor: i32,
    pub texture: i32,
    pub calories: i32,
}

#[derive(Debug, thiserror::Error)]
pub enum FromStrIntoIngredientError {
    #[error("Name not provided")]
    NameNotProvided,
    #[error("Invalid property pair: {0}")]
    InvalidPropertyPair(String),
    #[error("Invalid ingredient {0}")]
    InvalidIngridientName(String),
    #[error("Failed to parse int: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Ingredient not found")]
    IngridientNotFound(&'static str),
}

impl FromStr for Ingredient {
    type Err = FromStrIntoIngredientError;

    /// Example string:
    ///
    /// Sprinkles: capacity 2, durability 0, flavor -2, texture 0, calories 3
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon = s
            .find(':')
            .ok_or(FromStrIntoIngredientError::NameNotProvided)?;
        let name = s[..colon].trim().to_owned();
        if name.len() == 0 {
            return Err(FromStrIntoIngredientError::NameNotProvided);
        }

        let properties = s[colon + 1..].trim().split(", ").collect::<Vec<_>>();

        let mut capacity = None;
        let mut durability = None;
        let mut flavor = None;
        let mut texture = None;
        let mut calories = None;

        for pair in properties {
            let split = pair.split_whitespace().collect::<Vec<_>>();
            let &[prop, score] = split.as_slice() else {
                return Err(FromStrIntoIngredientError::InvalidPropertyPair(
                    pair.to_owned(),
                ));
            };
            match prop {
                "capacity" => {
                    capacity = Some(score.parse()?);
                }
                "durability" => {
                    durability = Some(score.parse()?);
                }
                "flavor" => {
                    flavor = Some(score.parse()?);
                }
                "texture" => {
                    texture = Some(score.parse()?);
                }
                "calories" => {
                    calories = Some(score.parse()?);
                }
                _ => {
                    return Err(FromStrIntoIngredientError::InvalidIngridientName(
                        prop.to_owned(),
                    ));
                }
            }
        }
        Ok(Ingredient {
            name,
            capacity: capacity.ok_or(FromStrIntoIngredientError::IngridientNotFound("capacity"))?,
            durability: durability
                .ok_or(FromStrIntoIngredientError::IngridientNotFound("durability"))?,
            flavor: flavor.ok_or(FromStrIntoIngredientError::IngridientNotFound("flavor"))?,
            texture: texture.ok_or(FromStrIntoIngredientError::IngridientNotFound("texture"))?,
            calories: calories.ok_or(FromStrIntoIngredientError::IngridientNotFound("calories"))?,
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    mod ingredient {
        use super::*;

        mod from_str {
            use super::*;

            #[test]
            fn valid_str() {
                let ingredient = Ingredient::from_str(
                    "Sprinkles: capacity 2, durability 0, flavor -2, texture 1, calories 3",
                )
                .unwrap();
                assert_eq!(
                    ingredient,
                    Ingredient {
                        name: "Sprinkles".to_owned(),
                        capacity: 2,
                        durability: 0,
                        flavor: -2,
                        texture: 1,
                        calories: 3
                    }
                )
            }

            #[test]
            fn lack_of_name() {
                let ingredient = Ingredient::from_str(
                    "   : capacity 2, durability 0, flavor -2, texture 1, calories 3",
                );
                assert!(ingredient.is_err())
            }

            #[test]
            fn lack_of_capacity() {
                let ingredient = Ingredient::from_str(
                    "Sprinkles: _capacity 2, durability 0, flavor -2, texture 1, calories 3",
                );
                assert!(ingredient.is_err())
            }

            #[test]
            fn lack_of_durability() {
                let ingredient = Ingredient::from_str(
                    "Sprinkles: capacity 2, _durability 0, flavor -2, texture 1, calories 3",
                );
                assert!(ingredient.is_err())
            }

            #[test]
            fn lack_of_flavor() {
                let ingredient = Ingredient::from_str(
                    "Sprinkles: capacity 2, durability 0, _flavor -2, texture 1, calories 3",
                );
                assert!(ingredient.is_err())
            }

            #[test]
            fn lack_of_texture() {
                let ingredient = Ingredient::from_str(
                    "Sprinkles: capacity 2, durability 0, flavor -2, _texture 1, calories 3",
                );
                assert!(ingredient.is_err())
            }

            #[test]
            fn lack_of_calories() {
                let ingredient = Ingredient::from_str(
                    "Sprinkles: capacity 2, durability 0, flavor -2, texture 1, _calories 3",
                );
                assert!(ingredient.is_err())
            }
        }
    }
}
