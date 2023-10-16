use crate::{circuit, wiring};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] wiring::from_str::ParseError),
    #[error("Circuit compute error: {0}")]
    Compute(#[from] circuit::ComputeError),
}
