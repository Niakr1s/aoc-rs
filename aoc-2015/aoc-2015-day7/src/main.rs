use std::{io::BufRead, path::PathBuf};

use aoc_2015_day7::{
    circuit::Circuit,
    error,
    wiring::{GateOrNumber, Number, Op, Wire},
};

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

    let mut circuit1 = Circuit::new();
    for line in reader.lines() {
        let line = line?;
        let wire: Wire = line.parse()?;
        circuit1.set(wire);
    }
    let circuit2 = circuit1.clone();

    let a1 = run_part1(circuit1)?;
    println!("Part1: contents of a is {a1}");
    let a2 = run_part2(circuit2, a1)?;
    println!("Part2: contents of a is {a2}");
    Ok(())
}

/// what signal is ultimately provided to wire a?
/// Param circuit should be circuit in initial state.
fn run_part1(mut circuit: Circuit) -> Result<u16, error::Error> {
    Ok(circuit.get(&"a".into())?)
}

/// Now, take the signal you got on wire a, override wire b to that signal,
/// and reset the other wires (including wire a).
/// What new signal is ultimately provided to wire a?
/// Param circuit should be circuit in initial state.
fn run_part2(mut circuit: Circuit, a_signal: u16) -> Result<u16, error::Error> {
    circuit.set(Wire {
        op: Op::GateOrNumber(GateOrNumber::Number(Number(a_signal))),
        target: "b".into(),
    });
    Ok(circuit.get(&"a".into())?)
}
