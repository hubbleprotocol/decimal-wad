use crate::error::*;

/// Scale of precision
pub const SCALE: usize = 18;
/// Identity
pub const WAD: u64 = 1_000_000_000_000_000_000;
/// Half of identity
pub const HALF_WAD: u64 = 500_000_000_000_000_000;
/// Scale for percentages
pub const PERCENT_SCALER: u64 = 10_000_000_000_000_000;
/// Scale for rpt
pub const RPT_SCALER: u128 = 1_000_000_000_000_000;

/// Scale for bips (100 bp = 1 percent)
pub const BPS_SCALER: u64 = PERCENT_SCALER / 100;

/// Try to subtract, return an error on underflow
pub trait TrySub: Sized {
    /// Subtract
    fn try_sub(self, rhs: Self) -> Result<Self, DecimalError>;
}

/// Try to subtract, return an error on overflow
pub trait TryAdd: Sized {
    /// Add
    fn try_add(self, rhs: Self) -> Result<Self, DecimalError>;
}

/// Try to divide, return an error on overflow or divide by zero
pub trait TryDiv<RHS>: Sized {
    /// Divide
    fn try_div(self, rhs: RHS) -> Result<Self, DecimalError>;
}

/// Try to multiply, return an error on overflow
pub trait TryMul<RHS>: Sized {
    /// Multiply
    fn try_mul(self, rhs: RHS) -> Result<Self, DecimalError>;
}
