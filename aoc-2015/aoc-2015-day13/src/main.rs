use std::io::BufRead;

use aoc_2015_day13::relations::{Relation, Relations};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = helpers::get_filepath_from_args();
    let f = std::fs::File::open(filepath).unwrap();
    let lines = std::io::BufReader::new(f).lines();

    let mut relations = Relations::new();
    for line in lines {
        let line = line?;
        let relation = Relation::from_adventofcode_line(&line)?;
        relations.update_relation(relation);
    }
    Ok(())
}
