use std::io::BufRead;

use aoc_2015_day13::relations::{Relation, RelationMap};
use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = helpers::get_filepath_from_args();
    let f = std::fs::File::open(filepath).unwrap();
    let lines = std::io::BufReader::new(f).lines();

    let mut relation_map = RelationMap::new();
    for line in lines {
        let line = line?;
        let relation = Relation::from_adventofcode_line(&line)?;
        relation_map.update_relation(relation);
    }
    let res = find_optimal_table_happiness(&relation_map)?;
    println!("Part1: optimal happiness is {}", res);
    Ok(())
}

fn find_optimal_table_happiness(
    relation_map: &RelationMap,
) -> Result<i32, Box<dyn std::error::Error>> {
    let all = relation_map
        .participants()
        .values()
        .copied()
        .collect::<Vec<_>>();
    let len = all.len();

    let mut max: Option<i32> = None;
    for happiness in all
        .into_iter()
        .permutations(len)
        .unique()
        .map(|c| relation_map.calculate_happiness(c.as_slice()))
    {
        let happiness = happiness?;
        max = max.map_or(Some(*happiness), |x| Some(x.max(*happiness)));
    }
    Ok(max.unwrap_or_default())
}
