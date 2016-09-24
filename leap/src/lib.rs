struct Year {
    value: i32
}

impl Year {
    fn is_divisible(&self, divisor: i32) -> bool {
        self.value % divisor == 0
    }

    fn is_leap(&self) -> bool {
        self.is_divisible(4) && (!self.is_divisible(100) || self.is_divisible(400))
    }
}

pub fn is_leap_year(year: i32) -> bool {
    Year { value: year }.is_leap()
}

