use std::collections::HashMap;

use crate::wiring::{Gate, GateOrNumber, Number, Op, Wire};

#[derive(Clone)]
pub struct Circuit {
    gates: HashMap<Gate, Op>,
}

#[derive(Debug, thiserror::Error)]
pub enum ComputeError {
    #[error("Gate not found: {0:?}")]
    GateNotFound(Gate),
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            gates: HashMap::new(),
        }
    }

    pub fn get(&mut self, gate: &Gate) -> Result<u16, ComputeError> {
        let op = self
            .gates
            .get(gate)
            .ok_or(ComputeError::GateNotFound(gate.clone()))?;
        let op = op.clone();

        // println!("BEG: {:?} {:?}", gate, op);
        let res = match op {
            Op::GateOrNumber(GateOrNumber::Number(Number(n))) => n,
            _ => {
                let res = op.compute(self)?;
                self.gates.insert(
                    gate.clone(),
                    Op::GateOrNumber(GateOrNumber::Number(Number(res))),
                );
                res
            }
        };
        // println!("END: {:?} {:?} => {res:?}", gate, op);

        Ok(res)
    }

    pub fn set(&mut self, wire: Wire) {
        self.gates.insert(wire.target, wire.op);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod circuit {
        use super::*;

        mod get {
            use super::*;

            macro_rules! num {
                ($s:expr, $n: expr) => {
                    Wire {
                        op: Op::GateOrNumber(GateOrNumber::Number(Number($n))),
                        target: Gate($s.to_owned()),
                    }
                };
            }

            #[test]
            fn test_from_website() {
                let wires: &[&str] = &[
                    "123 -> x",
                    "456 -> y",
                    "x AND y -> d",
                    "x OR y -> e",
                    "x LSHIFT 2 -> f",
                    "y RSHIFT 2 -> g",
                    "NOT x -> h",
                    "NOT y -> i",
                ];
                let mut circuit = Circuit::new();
                for wire in wires {
                    circuit.set(wire.parse().unwrap());
                }
                assert_eq!(circuit.gates.len(), 8);
                assert_eq!(circuit.get(&"d".into()).unwrap(), 72);
                assert_eq!(circuit.get(&"e".into()).unwrap(), 507);
                assert_eq!(circuit.get(&"f".into()).unwrap(), 492);
                assert_eq!(circuit.get(&"g".into()).unwrap(), 114);
                assert_eq!(circuit.get(&"h".into()).unwrap(), 65412);
                assert_eq!(circuit.get(&"i".into()).unwrap(), 65079);
                assert_eq!(circuit.get(&"x".into()).unwrap(), 123);
                assert_eq!(circuit.get(&"y".into()).unwrap(), 456);
            }

            #[test]
            fn test_num() {
                let mut circuit = Circuit::new();
                let wire: Wire = "123 -> x".parse().unwrap();
                circuit.set(wire);
                assert_eq!(circuit.gates.len(), 1);
                assert_eq!(circuit.get(&"x".into()).unwrap(), 123);
            }

            #[test]
            fn test_and() {
                let mut circuit = Circuit::new();
                let (x, y) = (0b0101, 0b1011);
                circuit.set(num!("x", x));
                circuit.set(num!("y", y));
                let wire: Wire = "x AND y -> z".parse().unwrap();
                circuit.set(wire);
                assert_eq!(circuit.gates.len(), 3);
                assert_eq!(circuit.get(&"z".into()).unwrap(), 0b0001);
                assert_eq!(circuit.get(&"x".into()).unwrap(), x);
                assert_eq!(circuit.get(&"y".into()).unwrap(), y);
            }

            #[test]
            fn test_or() {
                let mut circuit = Circuit::new();
                let (x, y) = (0b0101, 0b1010);
                circuit.set(num!("x", x));
                circuit.set(num!("y", y));
                let wire: Wire = "x OR y -> z".parse().unwrap();
                circuit.set(wire);
                assert_eq!(circuit.gates.len(), 3);
                assert_eq!(circuit.get(&"z".into()).unwrap(), 0b1111);
                assert_eq!(circuit.get(&"x".into()).unwrap(), x);
                assert_eq!(circuit.get(&"y".into()).unwrap(), y);
            }

            #[test]
            fn test_lshift() {
                let mut circuit = Circuit::new();
                let x = 0b0101;
                circuit.set(num!("x", x));
                let wire: Wire = "x LSHIFT 1 -> y".parse().unwrap();
                circuit.set(wire);
                assert_eq!(circuit.gates.len(), 2);
                assert_eq!(circuit.get(&"y".into()).unwrap(), 0b1010);
            }

            #[test]
            fn test_rshift() {
                let mut circuit = Circuit::new();
                let x = 0b0101;
                circuit.set(num!("x", x));
                let wire: Wire = "x RSHIFT 1 -> z".parse().unwrap();
                circuit.set(wire);
                assert_eq!(circuit.gates.len(), 2);
                assert_eq!(circuit.get(&"z".into()).unwrap(), 0b0010);
            }

            #[test]
            fn test_not() {
                let mut circuit = Circuit::new();
                let x = 0b0101;
                circuit.set(num!("x", x));
                let wire: Wire = "NOT x -> z".parse().unwrap();
                circuit.set(wire);
                assert_eq!(circuit.gates.len(), 2);
                assert_eq!(circuit.get(&"z".into()).unwrap(), 0b1111_1111_1111_1010);
            }
        }
    }
}
