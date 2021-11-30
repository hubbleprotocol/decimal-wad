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
        self.numerator
            .checked_mul(amount)
            .unwrap()
            .checked_div(self.denominator)
            .unwrap()
    }
}
