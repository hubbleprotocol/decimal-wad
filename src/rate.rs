#![allow(clippy::assign_op_pattern)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::reversed_empty_ranges)]
#![allow(clippy::manual_range_contains)]

use std::{convert::TryFrom, fmt};
use uint::construct_uint;

use crate::common::*;
use crate::decimal::*;
use crate::error::*;

// U128 with 128 bits consisting of 2 x 64-bit words
construct_uint! {
    pub struct U128(2);
}

/// Small decimal values, precise to 18 digits
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Rate(pub U128);

impl Rate {
    /// One
    pub fn one() -> Self {
        Self(Self::wad())
    }

    /// Zero
    pub fn zero() -> Self {
        Self(U128::from(0))
    }

    // OPTIMIZE: use const slice when fixed in BPF toolchain
    fn wad() -> U128 {
        U128::from(WAD)
    }

    // OPTIMIZE: use const slice when fixed in BPF toolchain
    pub fn half() -> Self {
        Self(Self::half_wad())
    }

    // OPTIMIZE: use const slice when fixed in BPF toolchain
    fn half_wad() -> U128 {
        U128::from(HALF_WAD)
    }

    /// Create scaled decimal from percent value
    pub fn from_percent(percent: u8) -> Self {
        Self(U128::from(percent as u64 * PERCENT_SCALER))
    }

    /// Create scaled decimal from bps value
    pub fn from_bps(bps: u16) -> Self {
        Self(U128::from(bps as u64 * BPS_SCALER))
    }

    pub fn from_bps_u64(bps: u64) -> Self {
        Self(U128::from(bps * BPS_SCALER))
    }

    /// Return raw scaled value
    #[allow(clippy::wrong_self_convention)]
    pub fn to_scaled_val(&self) -> u128 {
        self.0.as_u128()
    }

    /// Create scaled decimal from percent value
    pub fn to_bps(&self) -> Result<u128, DecimalError> {
        u128::try_from(self.0 / BPS_SCALER).map_err(|_| DecimalError::MathOverflow)
    }

    /// Create decimal from scaled value
    pub fn from_scaled_val(scaled_val: u64) -> Self {
        Self(U128::from(scaled_val))
    }

    /// Round scaled decimal to u64
    pub fn try_round_u64(&self) -> Result<u64, DecimalError> {
        let rounded_val = Self::half_wad()
            .checked_add(self.0)
            .ok_or(DecimalError::MathOverflow)?
            .checked_div(Self::wad())
            .ok_or(DecimalError::MathOverflow)?;
        u64::try_from(rounded_val).map_err(|_| DecimalError::MathOverflow)
    }

    /// Calculates base^exp
    pub fn try_pow(&self, mut exp: u64) -> Result<Rate, DecimalError> {
        let mut base = *self;
        let mut ret = if exp % 2 != 0 {
            base
        } else {
            Rate(Self::wad())
        };

        while exp > 0 {
            exp /= 2;
            base = base.try_mul(base)?;

            if exp % 2 != 0 {
                ret = ret.try_mul(base)?;
            }
        }

        Ok(ret)
    }
}

impl fmt::Display for Rate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut scaled_val = self.0.to_string();
        if scaled_val.len() <= SCALE {
            scaled_val.insert_str(0, &vec!["0"; SCALE - scaled_val.len()].join(""));
            scaled_val.insert_str(0, "0.");
        } else {
            scaled_val.insert(scaled_val.len() - SCALE, '.');
        }
        f.write_str(&scaled_val)
    }
}

impl TryFrom<Decimal> for Rate {
    type Error = DecimalError;
    fn try_from(decimal: Decimal) -> Result<Self, Self::Error> {
        Ok(Self(U128::from(decimal.to_scaled_val()?)))
    }
}

impl TryAdd for Rate {
    fn try_add(self, rhs: Self) -> Result<Self, DecimalError> {
        Ok(Self(
            self.0
                .checked_add(rhs.0)
                .ok_or(DecimalError::MathOverflow)?,
        ))
    }
}

impl TrySub for Rate {
    fn try_sub(self, rhs: Self) -> Result<Self, DecimalError> {
        Ok(Self(
            self.0
                .checked_sub(rhs.0)
                .ok_or(DecimalError::MathOverflow)?,
        ))
    }
}

impl TryDiv<u64> for Rate {
    fn try_div(self, rhs: u64) -> Result<Self, DecimalError> {
        Ok(Self(
            self.0
                .checked_div(U128::from(rhs))
                .ok_or(DecimalError::MathOverflow)?,
        ))
    }
}

impl TryDiv<Rate> for Rate {
    fn try_div(self, rhs: Self) -> Result<Self, DecimalError> {
        Ok(Self(
            self.0
                .checked_mul(Self::wad())
                .ok_or(DecimalError::MathOverflow)?
                .checked_div(rhs.0)
                .ok_or(DecimalError::MathOverflow)?,
        ))
    }
}

impl TryMul<u64> for Rate {
    fn try_mul(self, rhs: u64) -> Result<Self, DecimalError> {
        Ok(Self(
            self.0
                .checked_mul(U128::from(rhs))
                .ok_or(DecimalError::MathOverflow)?,
        ))
    }
}

impl TryMul<Rate> for Rate {
    fn try_mul(self, rhs: Self) -> Result<Self, DecimalError> {
        Ok(Self(
            self.0
                .checked_mul(rhs.0)
                .ok_or(DecimalError::MathOverflow)?
                .checked_div(Self::wad())
                .ok_or(DecimalError::MathOverflow)?,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pow() {
        assert_eq!(Rate::one(), Rate::one().try_pow(u64::MAX).unwrap());
    }
}
