use bitflags::bitflags;
use std::str::FromStr;

bitflags! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Property: u8 {
        const Capacity = 1 << 0;
        const Durability = 1 << 1;
        const Flavor = 1 << 2;
        const Texture = 1 << 3;
        const Calories = 1 << 4;
    }
}

pub struct Cookie {
    pub ingridients: Vec<(Ingredient, u32)>,
}

impl Cookie {
    pub fn new(ingridients: Vec<(Ingredient, u32)>) -> Self {
        Self { ingridients }
    }

    pub fn score_without_calories(&self) -> u32 {
        self.score_incl(
            Property::Capacity | Property::Durability | Property::Flavor | Property::Texture,
        )
    }

    fn score_incl(&self, incl: Property) -> u32 {
        let mut props = vec![];
        if incl & Property::Capacity != Property::empty() {
            props.push(self.capacity());
        }
        if incl & Property::Durability != Property::empty() {
            props.push(self.durability());
        }
        if incl & Property::Flavor != Property::empty() {
            props.push(self.flavor());
        }
        if incl & Property::Texture != Property::empty() {
            props.push(self.texture());
        }
        if incl & Property::Calories != Property::empty() {
            props.push(self.calories());
        }
        props.into_iter().product()
    }

    fn prop<F>(&self, mut prop_extractor: F) -> u32
    where
        F: FnMut(&Ingredient) -> i32,
    {
        self.ingridients
            .iter()
            .map(|(i, c)| prop_extractor(i) * (*c as i32))
            .sum::<i32>()
            .max(0) as u32
    }

    pub fn capacity(&self) -> u32 {
        self.prop(|i| i.capacity)
    }

    pub fn durability(&self) -> u32 {
        self.prop(|i| i.durability)
    }

    pub fn flavor(&self) -> u32 {
        self.prop(|i| i.flavor)
    }

    pub fn texture(&self) -> u32 {
        self.prop(|i| i.texture)
    }

    pub fn calories(&self) -> u32 {
        self.prop(|i| i.calories)
    }
}

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

    mod cookie {
        use super::*;

        fn cookie() -> Cookie {
            Cookie::new(vec![
                (
                    Ingredient::from_str(
                        "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
                    )
                    .unwrap(),
                    44,
                ),
                (
                    Ingredient::from_str(
                        "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
                    )
                    .unwrap(),
                    56,
                ),
            ])
        }

        #[test]
        fn capacity() {
            let cookie = cookie();
            assert_eq!(cookie.capacity(), 68);
        }

        #[test]
        fn durability() {
            let cookie = cookie();
            assert_eq!(cookie.durability(), 80);
        }

        #[test]
        fn flavor() {
            let cookie = cookie();
            assert_eq!(cookie.flavor(), 152);
        }

        #[test]
        fn texture() {
            let cookie = cookie();
            assert_eq!(cookie.texture(), 76);
        }

        #[test]
        fn calories() {
            let cookie = cookie();
            assert_eq!(cookie.calories(), 520);
        }

        #[test]
        fn score_without_calories() {
            let cookie = cookie();
            assert_eq!(cookie.score_without_calories(), 62842880);
        }

        #[test]
        fn score() {
            let cookie = cookie();
            assert_eq!(
                cookie.score_incl(
                    Property::Capacity
                        | Property::Durability
                        | Property::Flavor
                        | Property::Texture
                ),
                62842880
            );
        }

        #[test]
        fn score_durability_is_negative() {
            let mut cookie = cookie();
            // making butterscotch count to 150, so cookie's durability will be (150 * -2) + 56 * 3 = -132
            cookie.ingridients[0].1 = 150;
            assert_eq!(cookie.durability(), 0);
            assert_eq!(cookie.score_without_calories(), 0);
        }
    }

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
