use std::collections::HashMap;

use crate::ops::{Cmd, Gate, GateOrNumber, Number, Op};

pub struct GatePool {
    gates: HashMap<Gate, Op>,
}

#[derive(Debug)]
pub enum ComputeError {
    GateNotFound(Gate),
}

impl GatePool {
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

    pub fn set(&mut self, wire: Cmd) {
        self.gates.insert(wire.target, wire.op);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod gate_pool {
        use super::*;

        mod get {
            use super::*;

            macro_rules! num {
                ($s:expr, $n: expr) => {
                    Cmd {
                        op: Op::GateOrNumber(GateOrNumber::Number(Number($n))),
                        target: Gate($s.to_owned()),
                    }
                };
            }

            #[test]
            fn test_from_website() {
                let cmds: &[&str] = &[
                    "123 -> x",
                    "456 -> y",
                    "x AND y -> d",
                    "x OR y -> e",
                    "x LSHIFT 2 -> f",
                    "y RSHIFT 2 -> g",
                    "NOT x -> h",
                    "NOT y -> i",
                ];
                let mut pool = GatePool::new();
                for cmd in cmds {
                    pool.set(cmd.parse().unwrap());
                }
                assert_eq!(pool.gates.len(), 8);
                assert_eq!(pool.get(&"d".into()).unwrap(), 72);
                assert_eq!(pool.get(&"e".into()).unwrap(), 507);
                assert_eq!(pool.get(&"f".into()).unwrap(), 492);
                assert_eq!(pool.get(&"g".into()).unwrap(), 114);
                assert_eq!(pool.get(&"h".into()).unwrap(), 65412);
                assert_eq!(pool.get(&"i".into()).unwrap(), 65079);
                assert_eq!(pool.get(&"x".into()).unwrap(), 123);
                assert_eq!(pool.get(&"y".into()).unwrap(), 456);
            }

            #[test]
            fn test_num() {
                let mut pool = GatePool::new();
                let cmd: Cmd = "123 -> x".parse().unwrap();
                pool.set(cmd);
                assert_eq!(pool.gates.len(), 1);
                assert_eq!(pool.get(&"x".into()).unwrap(), 123);
            }

            #[test]
            fn test_and() {
                let mut pool = GatePool::new();
                let (x, y) = (0b0101, 0b1011);
                pool.set(num!("x", x));
                pool.set(num!("y", y));
                let cmd: Cmd = "x AND y -> z".parse().unwrap();
                pool.set(cmd);
                assert_eq!(pool.gates.len(), 3);
                assert_eq!(pool.get(&"z".into()).unwrap(), 0b0001);
                assert_eq!(pool.get(&"x".into()).unwrap(), x);
                assert_eq!(pool.get(&"y".into()).unwrap(), y);
            }

            #[test]
            fn test_or() {
                let mut pool = GatePool::new();
                let (x, y) = (0b0101, 0b1010);
                pool.set(num!("x", x));
                pool.set(num!("y", y));
                let cmd: Cmd = "x OR y -> z".parse().unwrap();
                pool.set(cmd);
                assert_eq!(pool.gates.len(), 3);
                assert_eq!(pool.get(&"z".into()).unwrap(), 0b1111);
                assert_eq!(pool.get(&"x".into()).unwrap(), x);
                assert_eq!(pool.get(&"y".into()).unwrap(), y);
            }

            #[test]
            fn test_lshift() {
                let mut pool = GatePool::new();
                let x = 0b0101;
                pool.set(num!("x", x));
                let cmd: Cmd = "x LSHIFT 1 -> y".parse().unwrap();
                pool.set(cmd);
                assert_eq!(pool.gates.len(), 2);
                assert_eq!(pool.get(&"y".into()).unwrap(), 0b1010);
            }

            #[test]
            fn test_rshift() {
                let mut pool = GatePool::new();
                let x = 0b0101;
                pool.set(num!("x", x));
                let cmd: Cmd = "x RSHIFT 1 -> z".parse().unwrap();
                pool.set(cmd);
                assert_eq!(pool.gates.len(), 2);
                assert_eq!(pool.get(&"z".into()).unwrap(), 0b0010);
            }

            #[test]
            fn test_not() {
                let mut pool = GatePool::new();
                let x = 0b0101;
                pool.set(num!("x", x));
                let cmd: Cmd = "NOT x -> z".parse().unwrap();
                pool.set(cmd);
                assert_eq!(pool.gates.len(), 2);
                assert_eq!(pool.get(&"z".into()).unwrap(), 0b1111_1111_1111_1010);
            }
        }
    }
}
