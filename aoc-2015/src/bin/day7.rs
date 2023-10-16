use std::{io::BufRead, path::PathBuf};

use lib::Cmd;

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

    let mut pool = lib::GatePool::new();
    for line in reader.lines() {
        let line = line?;
        let wire: Cmd = line.parse()?;
        pool.set(wire);
    }
    println!("Part1: contents of a is {}", pool.get(&"a".into()).unwrap());

    Ok(())
}

#[test]
fn test_run() {
    let filepath: PathBuf = std::path::PathBuf::from("d:/download/2015-day7.txt");
    run(filepath).unwrap();
}

mod error {
    use crate::lib;

    #[derive(Debug)]
    pub enum Error {
        Io(std::io::Error),
        Parse(lib::ParseError),
        Compute(lib::ComputeError),
    }

    impl From<std::io::Error> for Error {
        fn from(e: std::io::Error) -> Self {
            Error::Io(e)
        }
    }

    impl From<lib::ParseError> for Error {
        fn from(e: lib::ParseError) -> Self {
            Error::Parse(e)
        }
    }

    impl From<lib::ComputeError> for Error {
        fn from(e: lib::ComputeError) -> Self {
            Error::Compute(e)
        }
    }
}

mod lib {
    use std::{collections::HashMap, num::ParseIntError, str::FromStr};

    #[derive(Debug)]
    pub enum ComputeError {
        GateNotFound(Gate),
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Gate(String);

    impl From<&str> for Gate {
        fn from(value: &str) -> Self {
            Self(value.to_owned())
        }
    }

    pub struct GatePool {
        gates: HashMap<Gate, Op>,
    }

    impl GatePool {
        pub fn new() -> Self {
            Self {
                gates: HashMap::new(),
            }
        }

        pub fn get(&self, gate: &Gate) -> Result<u16, ComputeError> {
            self.gates
                .get(gate)
                .ok_or(ComputeError::GateNotFound(gate.clone()))?
                .compute(&self)
        }

        pub fn set(&mut self, wire: Cmd) {
            self.gates.insert(wire.target, wire.op);
        }
    }

    #[derive(Debug)]
    pub struct Cmd {
        pub op: Op,
        pub target: Gate,
    }

    #[derive(Debug, PartialEq)]
    pub enum GateOrNumber {
        Gate(Gate),
        Number(Number),
    }

    #[derive(Debug, PartialEq)]
    pub enum Op {
        GateOrNumber(GateOrNumber),
        Unary(UnaryOp),
        Binary(BinaryOp),
        Shift(ShiftOp),
    }

    #[derive(Debug, PartialEq)]
    enum UnaryOpKind {
        Not,
    }

    #[derive(Debug, PartialEq)]
    enum BinaryOpKind {
        And,
        Or,
    }

    #[derive(Debug, PartialEq)]
    enum ShiftOpKind {
        Lshift,
        Rshift,
    }

    #[derive(Debug, PartialEq)]
    pub struct Number(u16);

    #[derive(Debug, PartialEq)]
    pub struct UnaryOp {
        kind: UnaryOpKind,
        gate: Gate,
    }

    #[derive(Debug, PartialEq)]
    pub struct BinaryOp {
        kind: BinaryOpKind,
        lhs: GateOrNumber,
        rhs: Gate,
    }

    #[derive(Debug, PartialEq)]
    pub struct ShiftOp {
        kind: ShiftOpKind,
        lhs: Gate,
        rhs: Number,
    }

    impl UnaryOpKind {
        fn compute(&self, num: u16) -> u16 {
            match self {
                UnaryOpKind::Not => !num,
            }
        }
    }

    impl BinaryOpKind {
        fn compute(&self, lhs: u16, rhs: u16) -> u16 {
            match self {
                BinaryOpKind::And => lhs & rhs,
                BinaryOpKind::Or => lhs | rhs,
            }
        }
    }

    impl ShiftOpKind {
        fn compute(&self, lhs: u16, rhs: u16) -> u16 {
            match self {
                ShiftOpKind::Lshift => lhs << rhs,
                ShiftOpKind::Rshift => lhs >> rhs,
            }
        }
    }

    impl Number {
        fn compute(&self) -> u16 {
            self.0
        }
    }

    impl GateOrNumber {
        fn get_from_pool(&self, pool: &GatePool) -> Result<u16, ComputeError> {
            match self {
                GateOrNumber::Gate(gate) => pool.get(gate),
                GateOrNumber::Number(num) => Ok(num.0),
            }
        }
    }

    impl Gate {
        fn get_from_pool(&self, pool: &GatePool) -> Result<u16, ComputeError> {
            pool.get(self)
        }
    }

    impl UnaryOp {
        fn compute(&self, pool: &GatePool) -> Result<u16, ComputeError> {
            let num = pool.get(&self.gate)?;
            Ok(self.kind.compute(num))
        }
    }

    impl BinaryOp {
        fn compute(&self, pool: &GatePool) -> Result<u16, ComputeError> {
            let lhs = self.lhs.get_from_pool(pool)?;
            let rhs = self.rhs.get_from_pool(pool)?;
            Ok(self.kind.compute(lhs, rhs))
        }
    }

    impl ShiftOp {
        fn compute(&self, pool: &GatePool) -> Result<u16, ComputeError> {
            let lhs = pool.get(&self.lhs)?;
            Ok(self.kind.compute(lhs, self.rhs.0))
        }
    }

    impl Op {
        fn compute(&self, pool: &GatePool) -> Result<u16, ComputeError> {
            let res = match self {
                Op::GateOrNumber(gate_or_num) => match gate_or_num {
                    GateOrNumber::Gate(gate) => pool.get(gate)?,
                    GateOrNumber::Number(num) => num.compute(),
                },
                Op::Unary(op) => op.compute(pool)?,
                Op::Binary(op) => op.compute(pool)?,
                Op::Shift(op) => op.compute(pool)?,
            };
            Ok(res)
        }
    }

    #[derive(Debug)]
    pub struct ParseError {
        pub kind: ParseErrorKind,
        pub string: String,
    }

    #[derive(Debug)]
    pub enum ParseErrorKind {
        NoArrow,
        InvalidLength,
        InvalidUnaryType,
        InvalidBinaryType,
        ParseIntError(ParseIntError),
    }

    impl FromStr for Cmd {
        type Err = ParseError;

        /// Input examples: 123, NOT x, x AND y
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let split = s.split("->").collect::<Vec<_>>();
            if split.len() != 2 {
                return Err(ParseError {
                    kind: ParseErrorKind::NoArrow,
                    string: s.to_owned(),
                });
            }

            let op: Op = split[0].parse()?;
            let target: Gate = split[1].parse()?;

            Ok(Cmd { op, target })
        }
    }

    impl FromStr for Op {
        type Err = ParseError;

        /// Input examples: 123, NOT x, x AND y
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let s = s.trim();
            let split = s.split_whitespace().collect::<Vec<_>>();
            let op = match split.len() {
                1 => Op::GateOrNumber(s.parse()?),
                2 => Op::Unary(s.parse()?),
                3 => {
                    let last = *split.last().unwrap();
                    if last.parse::<u16>().is_ok() {
                        Op::Shift(s.parse()?)
                    } else {
                        Op::Binary(s.parse()?)
                    }
                }
                _ => {
                    return Err(ParseError {
                        kind: ParseErrorKind::InvalidLength,
                        string: s.to_owned(),
                    })
                }
            };
            Ok(op)
        }
    }

    impl FromStr for Number {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let s = s.trim();
            let num = Number(s.trim().parse::<u16>().map_err(|e| ParseError {
                kind: ParseErrorKind::ParseIntError(e),
                string: s.to_owned(),
            })?);
            Ok(num)
        }
    }

    impl FromStr for GateOrNumber {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let s = s.trim();
            let n = s.parse::<Number>();
            let res = match n {
                Ok(num) => GateOrNumber::Number(num),
                Err(_) => GateOrNumber::Gate(s.parse()?),
            };
            Ok(res)
        }
    }

    impl FromStr for UnaryOpKind {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.trim() {
                "NOT" => Ok(UnaryOpKind::Not),
                _ => Err(ParseError {
                    kind: ParseErrorKind::InvalidUnaryType,
                    string: s.to_owned(),
                }),
            }
        }
    }

    impl FromStr for BinaryOpKind {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.trim() {
                "AND" => Ok(BinaryOpKind::And),
                "OR" => Ok(BinaryOpKind::Or),
                _ => Err(ParseError {
                    kind: ParseErrorKind::InvalidBinaryType,
                    string: s.to_owned(),
                }),
            }
        }
    }

    impl FromStr for ShiftOpKind {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.trim() {
                "LSHIFT" => Ok(ShiftOpKind::Lshift),
                "RSHIFT" => Ok(ShiftOpKind::Rshift),
                _ => Err(ParseError {
                    kind: ParseErrorKind::InvalidBinaryType,
                    string: s.to_owned(),
                }),
            }
        }
    }

    impl FromStr for UnaryOp {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let splitted = s.trim().split_whitespace().collect::<Vec<_>>();
            if splitted.len() != 2 {
                return Err(ParseError {
                    kind: ParseErrorKind::InvalidLength,
                    string: s.to_owned(),
                });
            }
            let kind: UnaryOpKind = splitted[0].parse()?;
            let gate: Gate = splitted[1].parse()?;
            Ok(UnaryOp { kind, gate })
        }
    }

    impl FromStr for BinaryOp {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let splitted = s.trim().split_whitespace().collect::<Vec<_>>();
            if splitted.len() != 3 {
                return Err(ParseError {
                    kind: ParseErrorKind::InvalidLength,
                    string: s.to_owned(),
                });
            }
            let gate: GateOrNumber = splitted[0].parse()?;
            let kind: BinaryOpKind = splitted[1].parse()?;
            let rgate: Gate = splitted[2].parse()?;
            Ok(BinaryOp {
                kind,
                lhs: gate,
                rhs: rgate,
            })
        }
    }

    impl FromStr for ShiftOp {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let splitted = s.trim().split_whitespace().collect::<Vec<_>>();
            if splitted.len() != 3 {
                return Err(ParseError {
                    kind: ParseErrorKind::InvalidLength,
                    string: s.to_owned(),
                });
            }
            let gate: Gate = splitted[0].parse()?;
            let kind: ShiftOpKind = splitted[1].parse()?;
            let rgate: Number = splitted[2].parse()?;
            Ok(ShiftOp {
                kind,
                lhs: gate,
                rhs: rgate,
            })
        }
    }

    impl FromStr for Gate {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Gate(s.trim().to_owned()))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        mod cmd {
            use super::*;

            mod gate_pool {
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
                    let cmd: Cmd = "x AND y -> x".parse().unwrap();
                    pool.set(cmd);
                    assert_eq!(pool.gates.len(), 2);
                    assert_eq!(pool.get(&"x".into()).unwrap(), 0b0001);
                    assert_eq!(pool.get(&"y".into()).unwrap(), y);
                }

                #[test]
                fn test_or() {
                    let mut pool = GatePool::new();
                    let (x, y) = (0b0101, 0b1010);
                    pool.set(num!("x", x));
                    pool.set(num!("y", y));
                    let cmd: Cmd = "x OR y -> x".parse().unwrap();
                    pool.set(cmd);
                    assert_eq!(pool.gates.len(), 2);
                    assert_eq!(pool.get(&"x".into()).unwrap(), 0b1111);
                    assert_eq!(pool.get(&"y".into()).unwrap(), y);
                }

                #[test]
                fn test_lshift() {
                    let mut pool = GatePool::new();
                    let x = 0b0101;
                    pool.set(num!("x", x));
                    let cmd: Cmd = "x LSHIFT 1 -> x".parse().unwrap();
                    pool.set(cmd);
                    assert_eq!(pool.gates.len(), 1);
                    assert_eq!(pool.get(&"x".into()).unwrap(), 0b1010);
                }

                #[test]
                fn test_rshift() {
                    let mut pool = GatePool::new();
                    let x = 0b0101;
                    pool.set(num!("x", x));
                    let cmd: Cmd = "x RSHIFT 1 -> x".parse().unwrap();
                    pool.set(cmd);
                    assert_eq!(pool.gates.len(), 1);
                    assert_eq!(pool.get(&"x".into()).unwrap(), 0b0010);
                }

                #[test]
                fn test_not() {
                    let mut pool = GatePool::new();
                    let x = 0b0101;
                    pool.set(num!("x", x));
                    let cmd: Cmd = "NOT x -> x".parse().unwrap();
                    pool.set(cmd);
                    assert_eq!(pool.gates.len(), 1);
                    assert_eq!(pool.get(&"x".into()).unwrap(), 0b1111_1111_1111_1010);
                }
            }

            mod from_str {
                use super::*;

                #[test]
                #[should_panic]
                fn test_no_arrow() {
                    "123 > x".parse::<Cmd>().unwrap();
                }

                #[test]
                fn test_gate_or_number_num() {
                    let cmd: Cmd = "123 -> x".parse().unwrap();
                    assert_eq!(cmd.op, Op::GateOrNumber(GateOrNumber::Number(Number(123))));
                    assert_eq!(cmd.target, Gate("x".to_owned()));
                }

                #[test]
                fn test_gate_or_number_gate() {
                    let cmd: Cmd = "y -> x".parse().unwrap();
                    assert_eq!(
                        cmd.op,
                        Op::GateOrNumber(GateOrNumber::Gate(Gate("y".to_owned())))
                    );
                    assert_eq!(cmd.target, Gate("x".to_owned()));
                }

                #[test]
                fn test_and_gate() {
                    let cmd: Cmd = "x AND y -> x".parse().unwrap();
                    assert_eq!(
                        cmd.op,
                        Op::Binary(BinaryOp {
                            kind: BinaryOpKind::And,
                            lhs: GateOrNumber::Gate(Gate("x".to_owned())),
                            rhs: Gate("y".to_owned()),
                        })
                    );
                    assert_eq!(cmd.target, Gate("x".to_owned()));
                }

                #[test]
                fn test_and_number() {
                    let cmd: Cmd = "1 AND y -> x".parse().unwrap();
                    assert_eq!(
                        cmd.op,
                        Op::Binary(BinaryOp {
                            kind: BinaryOpKind::And,
                            lhs: GateOrNumber::Number(Number(1)),
                            rhs: Gate("y".to_owned()),
                        })
                    );
                    assert_eq!(cmd.target, Gate("x".to_owned()));
                }

                #[test]
                fn test_or_gate() {
                    let cmd: Cmd = "x OR y -> x".parse().unwrap();
                    assert_eq!(
                        cmd.op,
                        Op::Binary(BinaryOp {
                            kind: BinaryOpKind::Or,
                            lhs: GateOrNumber::Gate(Gate("x".to_owned())),
                            rhs: Gate("y".to_owned()),
                        })
                    );
                    assert_eq!(cmd.target, Gate("x".to_owned()));
                }

                #[test]
                fn test_or_number() {
                    let cmd: Cmd = "1 OR y -> x".parse().unwrap();
                    assert_eq!(
                        cmd.op,
                        Op::Binary(BinaryOp {
                            kind: BinaryOpKind::Or,
                            lhs: GateOrNumber::Number(Number(1)),
                            rhs: Gate("y".to_owned()),
                        })
                    );
                    assert_eq!(cmd.target, Gate("x".to_owned()));
                }

                #[test]
                fn test_lshift() {
                    let cmd: Cmd = "x LSHIFT 2 -> x".parse().unwrap();
                    assert_eq!(
                        cmd.op,
                        Op::Shift(ShiftOp {
                            kind: ShiftOpKind::Lshift,
                            lhs: Gate("x".to_owned()),
                            rhs: Number(2),
                        })
                    );
                    assert_eq!(cmd.target, Gate("x".to_owned()));
                }

                #[test]
                #[should_panic]
                fn test_lshift_invalid() {
                    "x LSHIFT y".parse::<Op>().unwrap();
                }

                #[test]
                fn test_rshift() {
                    let cmd: Cmd = "x RSHIFT 2 -> x".parse().unwrap();
                    assert_eq!(
                        cmd.op,
                        Op::Shift(ShiftOp {
                            kind: ShiftOpKind::Rshift,
                            lhs: Gate("x".to_owned()),
                            rhs: Number(2),
                        })
                    );
                    assert_eq!(cmd.target, Gate("x".to_owned()));
                }

                #[test]
                #[should_panic]
                fn test_rshift_invalid() {
                    "x RSHIFT y".parse::<Op>().unwrap();
                }

                #[test]
                fn test_not() {
                    let cmd: Cmd = "NOT x -> x".parse().unwrap();
                    assert_eq!(
                        cmd.op,
                        Op::Unary(UnaryOp {
                            kind: UnaryOpKind::Not,
                            gate: Gate("x".to_owned()),
                        })
                    );
                    assert_eq!(cmd.target, Gate("x".to_owned()));
                }
            }
        }
    }
}
