use std::{collections::HashMap, io::BufRead, str::FromStr};

use aoc_2015_day16::aunt_sue::{facts::Facts, AuntSue};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = helpers::get_filepath_from_args();
    let file = std::fs::File::open(filepath)?;
    let reader = std::io::BufReader::new(file);

    let aunts = reader
        .lines()
        .flat_map(|l| l.map(|l| AuntSue::from_str(&l)))
        .collect::<Result<Vec<_>, _>>()?;

    let known_facts = Facts(HashMap::from([
        ("children".to_owned(), 3),
        ("cats".to_owned(), 7),
        ("samoyeds".to_owned(), 2),
        ("pomeranians".to_owned(), 3),
        ("akitas".to_owned(), 0),
        ("vizslas".to_owned(), 0),
        ("goldfish".to_owned(), 5),
        ("trees".to_owned(), 3),
        ("cars".to_owned(), 2),
        ("perfumes".to_owned(), 1),
    ]));

    let possible_aunts = aunts
        .iter()
        .filter(|&aunt| known_facts.possible_match(&aunt.facts))
        .map(|aunt| aunt.no)
        .collect::<Vec<_>>();

    println!("Part1: possible aunts are {:?}", possible_aunts);

    Ok(())
}
