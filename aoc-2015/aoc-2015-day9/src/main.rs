use std::{io::BufRead, str::FromStr};

use aoc_2015_day9::graph::{Edge, Graph};

fn main() {
    let filepath = helpers::get_filepath_from_args();
    let file = std::fs::File::open(filepath).unwrap();
    let reader = std::io::BufReader::new(file);

    let edges = reader
        .lines()
        .flat_map(|line| line.map(|line| Edge::from_str(&line)))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let graph = Graph::new(&edges);

    let paths = graph.get_all_paths();
    let shortest = paths.shortest().unwrap();
    println!("shortest dist: {}", shortest.dist());
}
