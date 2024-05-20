use std::ops::{Add, Div, Mul, Sub};

pub trait DefaultNumberValueTrait {
    fn default() -> Self;
    fn default_eps() -> Self;
}

pub trait BaseNumberTypeTrait:
    DefaultNumberValueTrait
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialOrd
    + Clone
    + Copy
{
    fn sqrt(self) -> Self;
    fn abs(self) -> Self;
}

impl DefaultNumberValueTrait for f32 {
    fn default() -> Self {
        0.0
    }

    fn default_eps() -> Self {
        1e-6
    }
}

impl DefaultNumberValueTrait for f64 {
    fn default() -> Self {
        0.0
    }

    fn default_eps() -> Self {
        1e-12
    }
}

impl BaseNumberTypeTrait for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn abs(self) -> Self {
        self.abs()
    }
}

impl BaseNumberTypeTrait for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn abs(self) -> Self {
        self.abs()
    }
}
