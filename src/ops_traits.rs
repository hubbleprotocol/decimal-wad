use std::ops::{Add, Div, Mul, Sub};

use crate::{
    common::{
        uint::{U128, U192},
        TryAdd, TryDiv, TryMul, TrySub,
    },
    decimal::Decimal,
    rate::Rate,
};

impl<T> Mul<T> for Decimal
where
    T: Into<U192>,
{
    type Output = Decimal;
    fn mul(self, rhs: T) -> Decimal {
        self.try_mul(rhs).unwrap()
    }
}

impl Mul<Decimal> for Decimal {
    type Output = Decimal;

    fn mul(self, rhs: Decimal) -> Decimal {
        self.try_mul(rhs).unwrap()
    }
}

impl Mul<Rate> for Decimal {
    type Output = Decimal;

    fn mul(self, rhs: Rate) -> Decimal {
        self.try_mul(Self::from(rhs)).unwrap()
    }
}

impl<T> Div<T> for Decimal
where
    T: Into<U192>,
{
    type Output = Decimal;
    fn div(self, rhs: T) -> Decimal {
        self.try_div(rhs).unwrap()
    }
}

impl Div<Decimal> for Decimal {
    type Output = Decimal;

    fn div(self, rhs: Decimal) -> Decimal {
        self.try_div(rhs).unwrap()
    }
}

impl Add<Decimal> for Decimal {
    type Output = Decimal;

    fn add(self, rhs: Decimal) -> Decimal {
        self.try_add(rhs).unwrap()
    }
}

impl Sub<Decimal> for Decimal {
    type Output = Decimal;

    fn sub(self, rhs: Decimal) -> Decimal {
        self.try_sub(rhs).unwrap()
    }
}

impl<T> Mul<T> for Rate
where
    T: Into<U128>,
{
    type Output = Rate;
    fn mul(self, rhs: T) -> Rate {
        self.try_mul(rhs).unwrap()
    }
}

impl Mul<Rate> for Rate {
    type Output = Rate;

    fn mul(self, rhs: Rate) -> Rate {
        self.try_mul(rhs).unwrap()
    }
}

impl<T> Div<T> for Rate
where
    T: Into<U128>,
{
    type Output = Rate;
    fn div(self, rhs: T) -> Rate {
        self.try_div(rhs).unwrap()
    }
}

impl Div<Rate> for Rate {
    type Output = Rate;

    fn div(self, rhs: Rate) -> Rate {
        self.try_div(rhs).unwrap()
    }
}

impl Add<Rate> for Rate {
    type Output = Rate;

    fn add(self, rhs: Rate) -> Rate {
        self.try_add(rhs).unwrap()
    }
}

impl Sub<Rate> for Rate {
    type Output = Rate;

    fn sub(self, rhs: Rate) -> Rate {
        self.try_sub(rhs).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        let a = Decimal::from(4);
        let b: u64 = 3;
        let c = a * b;
        assert_eq!(c, Decimal::from(12));
    }

    #[test]
    fn test_mul_decimal() {
        let a = Decimal::from(4);
        let b = Decimal::from(3);
        let c = a * b;
        assert_eq!(c, Decimal::from(12));
    }

    #[test]
    fn test_mul_rate() {
        let a = Decimal::from(4);
        let b = Rate::from_percent(3);
        let c = a * b;
        assert_eq!(c, Decimal::from(12) / Decimal::from(100));
    }

    #[test]
    fn test_div_decimal() {
        let a = Decimal::from(12);
        let b = Decimal::from(3);
        let c = a / b;
        assert_eq!(c, Decimal::from(4));
    }

    #[test]
    fn test_add_decimal() {
        let a = Decimal::from(4);
        let b = Decimal::from(3);
        let c = a + b;
        assert_eq!(c, Decimal::from(7));
    }

    #[test]
    fn test_sub_decimal() {
        let a = Decimal::from(4);
        let b = Decimal::from(3);
        let c = a - b;
        assert_eq!(c, Decimal::from(1));
    }

    #[test]
    fn test_mul_decimal_large_values() {
        let a = Decimal::from(1_000_000);
        let b = Decimal::from(1_000_000);
        let c = a * b;
        assert_eq!(c, Decimal::from(1_000_000_000_000_u64));
    }

    #[test]
    fn test_mul_rate_large_values() {
        let a = Decimal::from(1_000_000);
        let b = Rate::from_percent(100);
        let c = a * b;
        assert_eq!(c, Decimal::from(1_000_000));
    }

    #[test]
    fn test_div_decimal_edge_case() {
        let a = Decimal::from(1);
        let b = Decimal::from(3);
        let c = a / b;
        assert_eq!(c.to_string(), "0.333333333333333333");
    }

    #[test]
    fn test_add_decimal_large_values() {
        let a = Decimal::from(1_000_000_000_000_u64);
        let b = Decimal::from(1_000_000_000_000_u64);
        let c = a + b;
        assert_eq!(c, Decimal::from(2_000_000_000_000_u64));
    }

    #[test]
    fn test_sub_decimal_edge_case() {
        let a = Decimal::from(1);
        let b = Decimal::from_scaled_val(999_999_999_999_999_999u64);
        let c = a - b;
        assert_eq!(c.to_string(), "0.000000000000000001");
    }
}
