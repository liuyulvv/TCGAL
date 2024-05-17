use std::ops::{Add, Div, Mul, Sub};

pub trait DefaultNumberValueTrait {
    fn default() -> Self;
    fn default_eps() -> Self;
}

pub trait BaseNumberTypeTrait:
    Add + Sub + Mul + Div + PartialOrd + Copy + DefaultNumberValueTrait
{
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

impl BaseNumberTypeTrait for f32 {}

impl BaseNumberTypeTrait for f64 {}
