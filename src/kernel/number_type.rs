use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg, Sub},
};

pub trait DefaultNumberValueTrait {
    fn default() -> Self;
    fn default_eps() -> Self;
    fn zero() -> Self;
    fn from_f64(value: f64) -> Self;
    fn pi() -> Self;
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
    + Display
{
    fn equals(self, other: Self) -> bool;
    fn sqrt(self) -> Self;
    fn abs(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
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

    fn pi() -> Self {
        std::f32::consts::PI
    }
}

impl DefaultNumberValueTrait for f64 {
    fn default() -> Self {
        0.0
    }

    fn default_eps() -> Self {
        1e-10
    }

    fn zero() -> Self {
        0.0
    }

    fn from_f64(value: f64) -> Self {
        value
    }

    fn pi() -> Self {
        std::f64::consts::PI
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

    fn sin(self) -> Self {
        self.sin()
    }

    fn cos(self) -> Self {
        self.cos()
    }

    fn acos(self) -> Self {
        self.acos()
    }

    fn atan(self) -> Self {
        self.atan()
    }

    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
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

    fn sin(self) -> Self {
        self.sin()
    }

    fn cos(self) -> Self {
        self.cos()
    }

    fn acos(self) -> Self {
        self.acos()
    }

    fn atan(self) -> Self {
        self.atan()
    }

    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }
}
