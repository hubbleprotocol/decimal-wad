use std::{convert::TryFrom, fmt};

use crate::common::*;
use crate::decimal::*;
use crate::error::*;

// Re-export for compatibility with pre 0.1.7 versions
pub use crate::common::uint::U128;

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
    pub fn from_percent<T>(percent: T) -> Self
    where
        T: Into<U128>,
    {
        let percent: U128 = percent.into();
        Self(percent.checked_mul(PERCENT_SCALER.into()).unwrap())
    }

    /// Create scaled decimal from bps value
    pub fn from_bps(bps: impl Into<U128>) -> Self {
        let bps: U128 = bps.into();
        Self(bps.checked_mul(BPS_SCALER.into()).unwrap())
    }

    #[deprecated(since = "0.1.7", note = "please use the generic `from_bps` instead")]
    pub fn from_bps_u64(bps: u64) -> Self {
        Self::from_bps(bps)
    }

    /// Return raw scaled value
    pub fn to_scaled_val<T>(&self) -> Result<T, DecimalError>
    where
        T: TryFrom<U128>,
    {
        T::try_from(self.0).map_err(|_| DecimalError::MathOverflow)
    }

    /// Create scaled decimal from percent value
    pub fn to_bps<T>(&self) -> Result<T, DecimalError>
    where
        T: TryFrom<U128>,
    {
        T::try_from(self.0 / BPS_SCALER).map_err(|_| DecimalError::MathOverflow)
    }

    /// Create decimal from scaled value
    pub fn from_scaled_val(scaled_val: impl Into<U128>) -> Self {
        Self(scaled_val.into())
    }

    /// Round scaled decimal
    pub fn try_round<T>(&self) -> Result<T, DecimalError>
    where
        T: TryFrom<U128>,
    {
        let rounded_val = Self::half_wad()
            .checked_add(self.0)
            .ok_or(DecimalError::MathOverflow)?
            .checked_div(Self::wad())
            .ok_or(DecimalError::MathOverflow)?;
        T::try_from(rounded_val).map_err(|_| DecimalError::MathOverflow)
    }

    /// Round scaled decimal to u64
    #[deprecated(since = "0.1.7", note = "please use the generic `try_round` instead")]
    pub fn try_round_u64(&self) -> Result<u64, DecimalError> {
        self.try_round()
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
        Ok(Self(decimal.to_scaled_val()?))
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

impl<T> TryDiv<T> for Rate
where
    T: Into<U128>,
{
    fn try_div(self, rhs: T) -> Result<Self, DecimalError> {
        Ok(Self(
            self.0
                .checked_div(rhs.into())
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

impl<T> TryMul<T> for Rate
where
    T: Into<U128>,
{
    fn try_mul(self, rhs: T) -> Result<Self, DecimalError> {
        Ok(Self(
            self.0
                .checked_mul(rhs.into())
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
