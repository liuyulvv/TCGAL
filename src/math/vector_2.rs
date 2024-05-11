use std::ops::{Add, Div, Mul, Sub};

use crate::traits::eps::Eps;

#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        let eps = Eps::default().value;
        if length < eps {
            return *self;
        }
        Self {
            x: self.x / length,
            y: self.y / length,
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &Self) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        let eps = Eps::default().value;
        (self.x - other.x).abs() < eps && (self.y - other.y).abs() < eps
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Self) -> Self::Output {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: Into<f64>> Mul<T> for Vector2 {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        let other = other.into();
        Self::new(self.x * other, self.y * other)
    }
}

impl<T: Into<f64>> Div<T> for Vector2 {
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        let other = other.into();
        Self::new(self.x / other, self.y / other)
    }
}
