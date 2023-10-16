use std::{io::BufRead, path::PathBuf};

use aoc_2015_day7::{circuit::Circuit, error, wiring::Wire};

fn main() -> Result<(), error::Error> {
    let filepath: PathBuf = std::env::args()
        .skip(1)
        .next()
        .expect("Provide a file path as first argument")
        .into();
    run(filepath)
}

fn run(filepath: PathBuf) -> Result<(), error::Error> {
    let file = std::fs::File::open(filepath)?;
    let reader = std::io::BufReader::new(file);

    let mut circuit = Circuit::new();
    for line in reader.lines() {
        let line = line?;
        let wire: Wire = line.parse()?;
        circuit.set(wire);
    }
    println!(
        "Part1: contents of a is {}",
        circuit.get(&"a".into()).unwrap()
    );

    Ok(())
}

mod lib {}
