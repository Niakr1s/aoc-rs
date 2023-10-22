use std::io::BufRead;

use aoc_2015_day14::{race::Race, reindeer::Reindeer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = helpers::get_filepath_from_args();
    let file = std::fs::File::open(filepath)?;
    let reader = std::io::BufReader::new(file);
    let reindeers = reader
        .lines()
        .flat_map(|line| line.map(|line| Reindeer::from_aoc_line(&line)))
        .collect::<Result<Vec<_>, _>>()?;
    let distances = Race::get_distances_after(&reindeers, 2503);
    let (winner, distance) = distances
        .iter()
        .enumerate()
        .max_by_key(|(_, &d)| d)
        .ok_or("No winner")?;
    println!(
        "Part1: the winner is {} with a distance of {}",
        reindeers[winner].name, distance
    );

    Ok(())
}
