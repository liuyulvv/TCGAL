use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

pub trait DefaultNumberValueTrait {
    fn default() -> Self;
    fn default_eps() -> Self;
    fn zero() -> Self;
    fn from_f64(value: f64) -> Self;
}

pub trait NumberType:
    DefaultNumberValueTrait
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + PartialOrd
    + Clone
    + Copy
    + Debug
{
    fn equals(self, other: Self) -> bool;
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

    fn zero() -> Self {
        0.0
    }

    fn from_f64(value: f64) -> Self {
        value as f32
    }
}

impl DefaultNumberValueTrait for f64 {
    fn default() -> Self {
        0.0
    }

    fn default_eps() -> Self {
        1e-12
    }

    fn zero() -> Self {
        0.0
    }

    fn from_f64(value: f64) -> Self {
        value
    }
}

impl NumberType for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn equals(self, other: Self) -> bool {
        let abs_diff = (self - other).abs();
        abs_diff < Self::default_eps()
    }
}

impl NumberType for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn equals(self, other: Self) -> bool {
        let abs_diff = (self - other).abs();
        abs_diff < Self::default_eps()
    }
}
