use std::io::BufRead;

use aoc_2015_day9::graph::{Edge, Graph};

fn main() {
    let filepath = helpers::get_filepath_from_args();
    let file = std::fs::File::open(filepath).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut graph = Graph::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let edge: Edge = line.parse().unwrap();
        graph.add_edge(edge);
    }

    let paths = graph.get_all_paths();
    let shortest = paths.shortest().unwrap();
    println!("shortest dist: {}", shortest.dist());
}
