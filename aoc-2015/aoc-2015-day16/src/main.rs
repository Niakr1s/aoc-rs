use std::{cmp::Ordering, collections::HashMap, io::BufRead, str::FromStr};

use aoc_2015_day16::aunt_sue::{
    facts::{Facts, FactsMatcher},
    AuntSue,
};

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

    let mut matcher = FactsMatcher::new();

    let possible_aunts = find_possible_aunts(aunts.iter(), &known_facts, &matcher);
    println!(
        "Part1: found {} possible aunts: {:?}",
        possible_aunts.len(),
        possible_aunts
    );

    matcher = matcher
        .with_ord("cats", Ordering::Greater)
        .with_ord("trees", Ordering::Greater)
        .with_ord("pomeranians", Ordering::Less)
        .with_ord("goldfish", Ordering::Less);
    let possible_aunts = find_possible_aunts(aunts.iter(), &known_facts, &matcher);
    println!(
        "Part2: found {} possible aunts: {:?}",
        possible_aunts.len(),
        possible_aunts
    );

    Ok(())
}

fn find_possible_aunts<'a>(
    aunts: impl Iterator<Item = &'a AuntSue>,
    known_facts: &Facts,
    matcher: &FactsMatcher,
) -> Vec<u32> {
    aunts
        .filter(|&aunt| matcher.is_possible_match(&known_facts, &aunt.facts))
        .map(|aunt| aunt.no)
        .collect()
}
