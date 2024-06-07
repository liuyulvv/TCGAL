use std::ops::{Add, Div, Mul, Sub};

use super::number_type::NumberType;

#[derive(Debug, Clone, Copy)]
pub struct Vector2<T: NumberType> {
    x: T,
    y: T,
}

impl<T: NumberType> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn length(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
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

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &Self) -> T {
        self.x * other.y - self.y * other.x
    }
}

impl<T: NumberType> Add for Vector2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: NumberType> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: NumberType> Mul<T> for Vector2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: NumberType> Div<T> for Vector2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: NumberType> PartialEq for Vector2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
