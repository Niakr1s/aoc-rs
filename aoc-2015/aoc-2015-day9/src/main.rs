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
    let graph = Graph::from_vec(&edges);

    let mut paths = graph.get_all_paths();
    paths.sort_by_dist();
    println!("Paths: {:?}", paths.len());

    println!("Part1: shortest dist: {}", paths.first().unwrap().dist());
    println!("Part2: longest dist: {}", paths.last().unwrap().dist());
}
