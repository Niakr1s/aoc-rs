use crate::gate_pool::{ComputeError, GatePool};

use super::*;

impl Gate {
    fn get_from_pool(&self, pool: &mut GatePool) -> Result<u16, ComputeError> {
        pool.get(self)
    }
}

impl GateOrNumber {
    fn get_from_pool(&self, pool: &mut GatePool) -> Result<u16, ComputeError> {
        match self {
            GateOrNumber::Gate(gate) => pool.get(gate),
            GateOrNumber::Number(num) => Ok(num.0),
        }
    }
}

impl Number {
    fn compute(&self) -> u16 {
        self.0
    }
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

impl UnaryOp {
    fn compute(&self, pool: &mut GatePool) -> Result<u16, ComputeError> {
        let num = pool.get(&self.gate)?;
        Ok(self.kind.compute(num))
    }
}

impl BinaryOp {
    fn compute(&self, pool: &mut GatePool) -> Result<u16, ComputeError> {
        let lhs = self.lhs.get_from_pool(pool)?;
        let rhs = self.rhs.get_from_pool(pool)?;
        Ok(self.kind.compute(lhs, rhs))
    }
}

impl ShiftOp {
    fn compute(&self, pool: &mut GatePool) -> Result<u16, ComputeError> {
        let lhs = pool.get(&self.lhs)?;
        Ok(self.kind.compute(lhs, self.rhs.0))
    }
}

impl Op {
    pub fn compute(&self, pool: &mut GatePool) -> Result<u16, ComputeError> {
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
