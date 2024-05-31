use std::ops::{Add, Div, Mul, Sub};

use super::number_type::NumberType;

#[derive(Debug, Clone, Copy)]
pub struct Vector2<NT: NumberType> {
    x: NT,
    y: NT,
}

impl<NT: NumberType> Vector2<NT> {
    pub fn new(x: NT, y: NT) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> NT {
        self.x
    }

    pub fn y(&self) -> NT {
        self.y
    }

    pub fn length(&self) -> NT {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        let eps = NT::default_eps();
        if length < eps {
            return *self;
        }
        Self {
            x: self.x / length,
            y: self.y / length,
        }
    }

    pub fn dot(&self, other: &Self) -> NT {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &Self) -> NT {
        self.x * other.y - self.y * other.x
    }
}

impl<NT: NumberType> Add for Vector2<NT> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl<NT: NumberType> Sub for Vector2<NT> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<NT: NumberType> Mul<NT> for Vector2<NT> {
    type Output = Self;

    fn mul(self, rhs: NT) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<NT: NumberType> Div<NT> for Vector2<NT> {
    type Output = Self;

    fn div(self, rhs: NT) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl<NT: NumberType> PartialEq for Vector2<NT> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
