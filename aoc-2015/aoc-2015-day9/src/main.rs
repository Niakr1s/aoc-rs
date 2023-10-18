use std::{collections::HashMap, io::BufRead, str::FromStr};

use aoc_2015_day9::my_graph;
use petgraph::{algo, prelude::*};

fn main() {
    let filepath = helpers::get_filepath_from_args();
    let file = std::fs::File::open(filepath).unwrap();
    let reader = std::io::BufReader::new(file);

    let edges = reader
        .lines()
        .flat_map(|line| line.map(|line| my_graph::Edge::from_str(&line)))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    for (i, (shortest, longest)) in [run_my(&edges), run_petagraph(&edges)].iter().enumerate() {
        println!("[{i}] shortest: {:?}, longest: {:?}", shortest, longest);
    }
}

fn run_my(edges: &Vec<my_graph::Edge>) -> (Option<u32>, Option<u32>) {
    let graph = my_graph::Graph::from_vec(edges);

    let mut paths = graph.get_all_paths();
    paths.sort_by_dist();

    (
        paths.first().map(|path| path.dist()),
        paths.last().map(|path| path.dist()),
    )
}

fn run_petagraph(edges: &Vec<my_graph::Edge>) -> (Option<u32>, Option<u32>) {
    let mut graph = petgraph::graph::UnGraph::<String, u32>::new_undirected();

    let mut edges_map: HashMap<&str, NodeIndex<u32>> = HashMap::new();

    for edge in edges {
        edges_map
            .entry(&edge.from)
            .or_insert_with(|| graph.add_node(edge.from.clone()));
        edges_map
            .entry(&edge.to)
            .or_insert_with(|| graph.add_node(edge.to.clone()));
    }

    for my_graph::Edge { from, to, dist } in edges {
        graph.update_edge(
            *edges_map.get(from.as_str()).unwrap(),
            *edges_map.get(to.as_str()).unwrap(),
            *dist,
        );
    }

    let (mut shortest, mut largest): (Option<u32>, Option<u32>) = (None, None);

    let edges = edges_map.drain().collect::<Vec<_>>();
    let wanted_intermediate_nodes = edges.len() - 2;
    for from_idx in 0..edges.len() {
        let from = edges[from_idx].1;

        for to_idx in from_idx + 1..edges.len() {
            let to = edges[to_idx].1;

            let paths = algo::all_simple_paths::<Vec<_>, _>(
                &graph,
                from,
                to,
                wanted_intermediate_nodes,
                Some(wanted_intermediate_nodes),
            )
            .collect::<Vec<_>>();

            for path in paths {
                let dist = path.windows(2).fold(0, |acc, item| {
                    let (a, b) = (item[0], item[1]);
                    let edge = graph.edges_connecting(a, b).next().unwrap();
                    acc + *edge.weight()
                });
                shortest = shortest.map(|s| s.min(dist)).or(Some(dist));
                largest = largest.map(|s| s.max(dist)).or(Some(dist));
            }
        }
    }
    (shortest, largest)
}
