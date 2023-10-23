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

    let best_score1 = find_best_score(ingridients.as_slice());
    println!("Part1: best score: {:?}", best_score1);

    let best_score2 = find_best_score_if(ingridients.as_slice(), |cookie| cookie.calories() == 500);
    println!(
        "Part2: best score for cookies with 500 calories: {:?}",
        best_score2
    );

    Ok(())
}

fn find_best_score_if(
    ingridients: &[Ingredient],
    predicate: impl Fn(&Cookie) -> bool,
) -> Option<u32> {
    (0..ingridients.len())
        .combinations_with_replacement(100)
        .filter_map(|c| {
            let ingridients = c
                .into_iter()
                .counts()
                .into_iter()
                .map(|c| (ingridients[c.0].clone(), c.1 as u32))
                .collect::<Vec<_>>();
            let cookie = Cookie::new(ingridients);
            if predicate(&cookie) {
                Some(cookie.score_without_calories())
            } else {
                None
            }
        })
        .max()
}

fn find_best_score(ingridients: &[Ingredient]) -> Option<u32> {
    find_best_score_if(ingridients, |_| true)
}
