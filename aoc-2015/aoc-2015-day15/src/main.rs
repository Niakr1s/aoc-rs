use std::io::BufRead;

use aoc_2015_day15::{Cookie, Ingredient};
use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = helpers::get_filepath_from_args();
    let f = std::fs::File::open(filepath)?;
    let reader = std::io::BufReader::new(f);

    let ingridients = reader
        .lines()
        .flat_map(|l| l.map(|l| l.parse::<Ingredient>()))
        .collect::<Result<Vec<Ingredient>, _>>()?;

    let max = (0..ingridients.len())
        .combinations_with_replacement(100)
        .map(|c| {
            let ingridients = c
                .into_iter()
                .counts()
                .into_iter()
                .map(|c| (ingridients[c.0].clone(), c.1 as u32))
                .collect::<Vec<_>>();
            let cookie = Cookie::new(ingridients);
            cookie.score_without_calories()
        })
        .max();
    println!("Max score without calories: {:?}", max);

    Ok(())
}
