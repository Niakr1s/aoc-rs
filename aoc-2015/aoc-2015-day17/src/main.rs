use std::io::BufRead;

use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = helpers::get_filepath_from_args();
    let file = std::fs::File::open(filepath)?;
    let containers = get_numbers(file)?;

    let combinations = containers
        .into_iter()
        .powerset()
        .filter(|c| c.iter().sum::<u32>() == 150);
    let count = combinations.count();
    println!("Part 1: {}", count);

    Ok(())
}

fn get_numbers(input: impl std::io::Read) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let reader = std::io::BufReader::new(input);
    let ret = reader
        .lines()
        .flat_map(|line| line.map(|line| line.parse::<u32>()))
        .collect::<Result<Vec<u32>, _>>()?;
    Ok(ret)
}
