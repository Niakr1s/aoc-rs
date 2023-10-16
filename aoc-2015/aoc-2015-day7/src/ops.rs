pub mod compute;
pub mod from;
pub mod from_str;

use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct Cmd {
    pub op: Op,
    pub target: Gate,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    GateOrNumber(GateOrNumber),
    Unary(UnaryOp),
    Binary(BinaryOp),
    Shift(ShiftOp),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Number(pub u16);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Gate(pub String);

#[derive(Debug, PartialEq, Clone)]
pub enum GateOrNumber {
    Gate(Gate),
    Number(Number),
}

#[derive(Debug, PartialEq, Clone)]
enum UnaryOpKind {
    Not,
}

#[derive(Debug, PartialEq, Clone)]
enum BinaryOpKind {
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
enum ShiftOpKind {
    Lshift,
    Rshift,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOp {
    kind: UnaryOpKind,
    gate: Gate,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryOp {
    kind: BinaryOpKind,
    lhs: GateOrNumber,
    rhs: Gate,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ShiftOp {
    kind: ShiftOpKind,
    lhs: Gate,
    rhs: Number,
}
