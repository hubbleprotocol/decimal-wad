use std::{convert::TryFrom, fmt};

use crate::common::*;
use crate::error::*;
use crate::rate::*;

// Re-export for compatibility with pre 0.1.7 versions
pub use crate::common::uint::U192;

/// Large decimal values, precise to 18 digits
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Decimal(pub U192);

impl Decimal {
    /// One
    pub fn one() -> Self {
        Self(Self::wad())
    }

    /// Zero
    pub fn zero() -> Self {
        Self(U192::zero())
    }

    // OPTIMIZE: use const slice when fixed in BPF toolchain
    fn wad() -> U192 {
        U192::from(WAD)
    }

    // OPTIMIZE: use const slice when fixed in BPF toolchain
    fn half_wad() -> U192 {
        U192::from(HALF_WAD)
    }

    /// Create scaled decimal from percent value
    pub fn from_percent<T>(percent: T) -> Self
    where
        T: Into<U192>,
    {
        let percent: U192 = percent.into();
        Self(percent.checked_mul(PERCENT_SCALER.into()).unwrap())
    }

    #[deprecated(
        since = "0.1.7",
        note = "please use the generic `from_percent` instead"
    )]
    pub fn from_percent_u64(percent: u64) -> Self {
        Self::from_percent(percent)
    }

    /// Get percent value from a scaled decimal
    pub fn to_percent<T>(&self) -> Result<T, DecimalError>
    where
        T: TryFrom<U192>,
    {
        T::try_from(self.0 / PERCENT_SCALER).map_err(|_| DecimalError::MathOverflow)
    }

    /// Create scaled decimal from percent value
    pub fn to_bps<T>(&self) -> Result<T, DecimalError>
    where
        T: TryFrom<U192>,
    {
        T::try_from(self.0 / BPS_SCALER).map_err(|_| DecimalError::MathOverflow)
    }

    /// Create scaled decimal from bps value
    pub fn from_bps(bps: impl Into<U192>) -> Self {
        let bps: U192 = bps.into();
        Self(bps.checked_mul(BPS_SCALER.into()).unwrap())
    }

    /// Return raw scaled value if it fits the destination type T
    pub fn to_scaled_val<T>(&self) -> Result<T, DecimalError>
    where
        T: TryFrom<U192>,
    {
        T::try_from(self.0).map_err(|_| DecimalError::MathOverflow)
    }

    /// Create decimal from scaled value
    pub fn from_scaled_val(scaled_val: impl Into<U192>) -> Self {
        Self(scaled_val.into())
    }

    /// Round scaled decimal
    pub fn try_round<T>(&self) -> Result<T, DecimalError>
    where
        T: TryFrom<U192>,
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

    /// Round scaled decimal to u128
    #[deprecated(since = "0.1.7", note = "please use the generic `try_round` instead")]
    pub fn try_round_u128(&self) -> Result<u128, DecimalError> {
        self.try_round()
    }

    /// Ceiling scaled decimal
    pub fn try_ceil<T>(&self) -> Result<T, DecimalError>
    where
        T: TryFrom<U192>,
    {
        let ceil_val = Self::wad()
            .checked_sub(U192::from(1u64))
            .ok_or(DecimalError::MathOverflow)?
            .checked_add(self.0)
            .ok_or(DecimalError::MathOverflow)?
            .checked_div(Self::wad())
            .ok_or(DecimalError::MathOverflow)?;
        T::try_from(ceil_val).map_err(|_| DecimalError::MathOverflow)
    }

    /// Ceiling scaled decimal to u64
    #[deprecated(since = "0.1.7", note = "please use the generic `try_ceil` instead")]
    pub fn try_ceil_u64(&self) -> Result<u64, DecimalError> {
        self.try_ceil()
    }

    /// Ceiling scaled decimal to u128
    #[deprecated(since = "0.1.7", note = "please use the generic `try_ceil` instead")]
    pub fn try_ceil_u128(&self) -> Result<u128, DecimalError> {
        self.try_ceil()
    }

    /// Floor scaled decimal
    pub fn try_floor<T>(&self) -> Result<T, DecimalError>
    where
        T: TryFrom<U192>,
    {
        let ceil_val = self
            .0
            .checked_div(Self::wad())
            .ok_or(DecimalError::MathOverflow)?;
        T::try_from(ceil_val).map_err(|_| DecimalError::MathOverflow)
    }

    /// Floor scaled decimal to u64
    #[deprecated(since = "0.1.7", note = "please use the generic `try_floor` instead")]
    pub fn try_floor_u64(&self) -> Result<u64, DecimalError> {
        self.try_floor()
    }

    /// Floor scaled decimal to u128
    #[deprecated(since = "0.1.7", note = "please use the generic `try_floor` instead")]
    pub fn try_floor_u128(&self) -> Result<u128, DecimalError> {
        self.try_floor()
    }
}

impl fmt::Display for Decimal {
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

impl<T> From<T> for Decimal
where
    T: Into<U128>,
{
    fn from(val: T) -> Self {
        let val: U128 = val.into();
        // Note: Some values between `u64::MAX` and `u128::MAX` can overflow...so panics here
        Self(Self::wad().checked_mul(val.into()).unwrap())
    }
}

impl From<Rate> for Decimal {
    fn from(val: Rate) -> Self {
        Self(val.to_scaled_val().unwrap())
    }
}

impl TryAdd for Decimal {
    fn try_add(self, rhs: Self) -> Result<Self, DecimalError> {
        Ok(Self(
            self.0
                .checked_add(rhs.0)
                .ok_or(DecimalError::MathOverflow)?,
        ))
    }
}

impl TrySub for Decimal {
    fn try_sub(self, rhs: Self) -> Result<Self, DecimalError> {
        Ok(Self(
            self.0
                .checked_sub(rhs.0)
                .ok_or(DecimalError::MathOverflow)?,
        ))
    }
}

impl<T> TryDiv<T> for Decimal
where
    T: Into<U192>,
{
    fn try_div(self, rhs: T) -> Result<Self, DecimalError> {
        Ok(Self(
            self.0
                .checked_div(rhs.into())
                .ok_or(DecimalError::MathOverflow)?,
        ))
    }
}

impl TryDiv<Rate> for Decimal {
    fn try_div(self, rhs: Rate) -> Result<Self, DecimalError> {
        self.try_div(Self::from(rhs))
    }
}

impl TryDiv<Decimal> for Decimal {
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

impl<T> TryMul<T> for Decimal
where
    T: Into<U192>,
{
    fn try_mul(self, rhs: T) -> Result<Self, DecimalError> {
        Ok(Self(
            self.0
                .checked_mul(rhs.into())
                .ok_or(DecimalError::MathOverflow)?,
        ))
    }
}

impl TryMul<Rate> for Decimal {
    fn try_mul(self, rhs: Rate) -> Result<Self, DecimalError> {
        self.try_mul(Self::from(rhs))
    }
}

impl TryMul<Decimal> for Decimal {
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
    fn test_scaler() {
        assert_eq!(U192::exp10(SCALE), Decimal::wad());
    }

    #[test]
    fn test_decimal_from_to_percent() {
        let pct = 10; // 10%
        let x = Decimal::from_percent(pct);
        let pct_actual = x.to_percent().unwrap();

        assert_eq!(pct as u128, pct_actual);
    }
}
