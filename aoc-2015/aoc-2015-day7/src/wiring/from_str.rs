use super::*;

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

impl FromStr for Wire {
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

        Ok(Wire { op, target })
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

    #[test]
    #[should_panic]
    fn test_no_arrow() {
        "123 > x".parse::<Wire>().unwrap();
    }

    #[test]
    fn test_gate_or_number_num() {
        let wire: Wire = "123 -> x".parse().unwrap();
        assert_eq!(wire.op, Op::GateOrNumber(GateOrNumber::Number(Number(123))));
        assert_eq!(wire.target, Gate("x".to_owned()));
    }

    #[test]
    fn test_gate_or_number_gate() {
        let wire: Wire = "y -> x".parse().unwrap();
        assert_eq!(
            wire.op,
            Op::GateOrNumber(GateOrNumber::Gate(Gate("y".to_owned())))
        );
        assert_eq!(wire.target, Gate("x".to_owned()));
    }

    #[test]
    fn test_and_gate() {
        let wire: Wire = "x AND y -> x".parse().unwrap();
        assert_eq!(
            wire.op,
            Op::Binary(BinaryOp {
                kind: BinaryOpKind::And,
                lhs: GateOrNumber::Gate(Gate("x".to_owned())),
                rhs: Gate("y".to_owned()),
            })
        );
        assert_eq!(wire.target, Gate("x".to_owned()));
    }

    #[test]
    fn test_and_number() {
        let wire: Wire = "1 AND y -> x".parse().unwrap();
        assert_eq!(
            wire.op,
            Op::Binary(BinaryOp {
                kind: BinaryOpKind::And,
                lhs: GateOrNumber::Number(Number(1)),
                rhs: Gate("y".to_owned()),
            })
        );
        assert_eq!(wire.target, Gate("x".to_owned()));
    }

    #[test]
    fn test_or_gate() {
        let wire: Wire = "x OR y -> x".parse().unwrap();
        assert_eq!(
            wire.op,
            Op::Binary(BinaryOp {
                kind: BinaryOpKind::Or,
                lhs: GateOrNumber::Gate(Gate("x".to_owned())),
                rhs: Gate("y".to_owned()),
            })
        );
        assert_eq!(wire.target, Gate("x".to_owned()));
    }

    #[test]
    fn test_or_number() {
        let wire: Wire = "1 OR y -> x".parse().unwrap();
        assert_eq!(
            wire.op,
            Op::Binary(BinaryOp {
                kind: BinaryOpKind::Or,
                lhs: GateOrNumber::Number(Number(1)),
                rhs: Gate("y".to_owned()),
            })
        );
        assert_eq!(wire.target, Gate("x".to_owned()));
    }

    #[test]
    fn test_lshift() {
        let wire: Wire = "x LSHIFT 2 -> x".parse().unwrap();
        assert_eq!(
            wire.op,
            Op::Shift(ShiftOp {
                kind: ShiftOpKind::Lshift,
                lhs: Gate("x".to_owned()),
                rhs: Number(2),
            })
        );
        assert_eq!(wire.target, Gate("x".to_owned()));
    }

    #[test]
    #[should_panic]
    fn test_lshift_invalid() {
        "x LSHIFT y".parse::<Op>().unwrap();
    }

    #[test]
    fn test_rshift() {
        let wire: Wire = "x RSHIFT 2 -> x".parse().unwrap();
        assert_eq!(
            wire.op,
            Op::Shift(ShiftOp {
                kind: ShiftOpKind::Rshift,
                lhs: Gate("x".to_owned()),
                rhs: Number(2),
            })
        );
        assert_eq!(wire.target, Gate("x".to_owned()));
    }

    #[test]
    #[should_panic]
    fn test_rshift_invalid() {
        "x RSHIFT y".parse::<Op>().unwrap();
    }

    #[test]
    fn test_not() {
        let wire: Wire = "NOT x -> x".parse().unwrap();
        assert_eq!(
            wire.op,
            Op::Unary(UnaryOp {
                kind: UnaryOpKind::Not,
                gate: Gate("x".to_owned()),
            })
        );
        assert_eq!(wire.target, Gate("x".to_owned()));
    }
}
