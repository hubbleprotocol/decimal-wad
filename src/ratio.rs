pub struct Ratio {
    pub numerator: u64,
    pub denominator: u64,
}

impl Ratio {
    pub fn new(numerator: u64, denominator: u64) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn mul(&self, amount: u64) -> u64 {
        (self.numerator as u128)
            .checked_mul(amount as u128)
            .unwrap()
            .checked_div(self.denominator as u128)
            .unwrap() as u64
    }
}
