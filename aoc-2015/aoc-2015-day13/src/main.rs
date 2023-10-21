use std::io::BufRead;

use aoc_2015_day13::relations::{Relation, Relations};
use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = helpers::get_filepath_from_args();
    let f = std::fs::File::open(filepath).unwrap();
    let lines = std::io::BufReader::new(f).lines();

    let mut relations = Relations::new();
    for line in lines {
        let line = line?;
        let relation = line_to_relation(&line)?;
        relations.update_relation(relation);
    }
    Ok(())
}

/// Alice would gain 54 happiness units by sitting next to Bob.
fn line_to_relation(mut line: &str) -> Result<Relation<&str, u8>, &'static str> {
    if line.ends_with(".") {
        line = &line[..line.len() - 1];
    }

    let v = line.split_whitespace().collect_vec();

    let (to, happiness, from) = (0, 3, 10);
    if v.len() != 11 {
        return Err("invalid line");
    } else {
        Ok(Relation {
            from: v[from],
            to: v[to],
            happiness: v[happiness].parse().unwrap(),
        })
    }
}
