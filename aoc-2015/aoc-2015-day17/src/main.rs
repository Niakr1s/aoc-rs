use std::io::BufRead;

use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = helpers::get_filepath_from_args();
    let file = std::fs::File::open(filepath)?;
    let containers = get_numbers(file)?;

    println!("Part 1: {}", combinations_count(&containers));
    println!("Part 2: {}", different_min_ways_count(&containers));

    Ok(())
}

fn combinations_count(containers: &[u32]) -> usize {
    containers
        .iter()
        .powerset()
        .filter(|c| c.into_iter().copied().sum::<u32>() == 150)
        .count()
}

fn different_min_ways_count(containers: &[u32]) -> usize {
    containers
        .iter()
        .powerset()
        .filter(|c| c.into_iter().copied().sum::<u32>() == 150)
        .min_set_by(|a, b| a.len().cmp(&b.len()))
        .len()
}

fn get_numbers(input: impl std::io::Read) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let reader = std::io::BufReader::new(input);
    let ret = reader
        .lines()
        .flat_map(|line| line.map(|line| line.parse::<u32>()))
        .collect::<Result<Vec<u32>, _>>()?;
    Ok(ret)
}
