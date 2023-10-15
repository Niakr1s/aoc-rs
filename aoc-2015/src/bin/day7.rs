fn main() {}

mod lib {
    use std::{collections::HashMap, num::ParseIntError, str::FromStr};

    pub enum ComputeError {
        GateNotFound(Gate),
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Gate(String);

    pub struct GatePool {
        gates: HashMap<Gate, u16>,
    }

    impl GatePool {
        pub fn new() -> Self {
            Self {
                gates: HashMap::new(),
            }
        }

        pub fn get(&self, gate: &Gate) -> Option<&u16> {
            self.gates.get(gate)
        }
    }

    struct Cmd {
        op: Op,
        target: Gate,
    }

    #[derive(Debug, PartialEq)]
    enum Op {
        Number(Number),
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
    struct Number(u16);

    #[derive(Debug, PartialEq)]
    struct UnaryOp {
        kind: UnaryOpKind,
        gate: Gate,
    }

    #[derive(Debug, PartialEq)]
    struct BinaryOp {
        kind: BinaryOpKind,
        lhs: Gate,
        rhs: Gate,
    }

    #[derive(Debug, PartialEq)]
    struct ShiftOp {
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
        fn new(num: u16) -> Self {
            Self(num)
        }

        fn compute(&self) -> u16 {
            self.0
        }
    }

    impl UnaryOp {
        fn compute(&self, pool: &GatePool) -> Result<u16, ComputeError> {
            let num = pool
                .get(&self.gate)
                .ok_or(ComputeError::GateNotFound(self.gate.clone()))?;
            Ok(self.kind.compute(*num))
        }
    }

    impl BinaryOp {
        fn compute(&self, pool: &GatePool) -> Result<u16, ComputeError> {
            let lhs = pool
                .get(&self.lhs)
                .ok_or(ComputeError::GateNotFound(self.lhs.clone()))?;
            let rhs = pool
                .get(&self.rhs)
                .ok_or(ComputeError::GateNotFound(self.rhs.clone()))?;
            Ok(self.kind.compute(*lhs, *rhs))
        }
    }

    impl ShiftOp {
        fn compute(&self, pool: &GatePool) -> Result<u16, ComputeError> {
            let lhs = pool
                .get(&self.lhs)
                .ok_or(ComputeError::GateNotFound(self.lhs.clone()))?;

            Ok(self.kind.compute(*lhs, self.rhs.0))
        }
    }

    impl Op {
        fn compute(&self, pool: &GatePool) -> Result<u16, ComputeError> {
            let res = match self {
                Op::Number(num) => num.compute(),
                Op::Unary(op) => op.compute(pool)?,
                Op::Binary(op) => op.compute(pool)?,
                Op::Shift(op) => op.compute(pool)?,
            };
            Ok(res)
        }
    }

    #[derive(Debug)]
    pub struct ParseError {
        kind: ParseErrorKind,
        string: String,
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
            let split = s.trim().split_whitespace().collect::<Vec<_>>();
            let op = match split.len() {
                1 => Op::Number(s.parse()?),
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
            let gate: Gate = splitted[0].parse()?;
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

            mod from_str {
                use super::*;

                #[test]
                #[should_panic]
                fn test_no_arrow() {
                    "123 > x".parse::<Cmd>().unwrap();
                }

                #[test]
                fn test_num() {
                    let cmd: Cmd = "123 -> x".parse().unwrap();
                    assert_eq!(cmd.op, Op::Number(Number(123)));
                    assert_eq!(cmd.target, Gate("x".to_owned()));
                }

                #[test]
                fn test_and() {
                    let cmd: Cmd = "x AND y -> x".parse().unwrap();
                    assert_eq!(
                        cmd.op,
                        Op::Binary(BinaryOp {
                            kind: BinaryOpKind::And,
                            lhs: Gate("x".to_owned()),
                            rhs: Gate("y".to_owned()),
                        })
                    );
                    assert_eq!(cmd.target, Gate("x".to_owned()));
                }

                #[test]
                fn test_or() {
                    let cmd: Cmd = "x OR y -> x".parse().unwrap();
                    assert_eq!(
                        cmd.op,
                        Op::Binary(BinaryOp {
                            kind: BinaryOpKind::Or,
                            lhs: Gate("x".to_owned()),
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
