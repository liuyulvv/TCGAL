use std::ops::{Add, Div, Mul, Sub};

use crate::{
    kernel::base_kernel::base_vector_2::BaseVector2,
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

#[derive(Debug, Clone, Copy)]
pub struct Vector2<T: BaseNumberTypeTrait> {
    x: T,
    y: T,
}

impl<T: BaseNumberTypeTrait> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T: BaseNumberTypeTrait> BaseVector2<T> for Vector2<T> {
    fn x(&self) -> T {
        self.x
    }

    fn y(&self) -> T {
        self.y
    }

    fn length(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(&self) -> Self {
        let length = self.length();
        let eps = T::default_eps();
        if length < eps {
            return *self;
        }
        Self {
            x: self.x / length,
            y: self.y / length,
        }
    }

    fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }

    fn cross(&self, other: &Self) -> T {
        self.x * other.y - self.y * other.x
    }
}

impl<T: BaseNumberTypeTrait> Add for Vector2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: BaseNumberTypeTrait> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: BaseNumberTypeTrait> Mul<T> for Vector2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: BaseNumberTypeTrait> Div<T> for Vector2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: BaseNumberTypeTrait> PartialEq for Vector2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
