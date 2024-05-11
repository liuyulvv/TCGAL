use std::ops::{Add, Div, Mul, Sub};

use crate::{
    math::vector_2::Vector2,
    traits::{eps::Eps, is_equal::IsEqual, is_same::IsSame},
};

#[derive(Debug, Clone, Copy)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

impl Point2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn from_vector(vector: &Vector2) -> Self {
        Self::new(vector.x, vector.y)
    }

    pub fn distance_to(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn get_vector(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
}

impl Default for Point2 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl IsEqual for Point2 {
    fn is_equal(&self, other: &Self, eps: Option<Eps>) -> bool {
        let eps = eps.unwrap_or(Eps::default()).value;
        let diff_x = self.x - other.x;
        let diff_y = self.y - other.y;
        diff_x.abs() < eps && diff_y.abs() < eps
    }
}

impl IsSame for Point2 {
    fn is_same(&self, other: &Self, eps: Option<Eps>) -> bool {
        self.is_equal(other, eps)
    }
}

impl PartialEq for Point2 {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other, None)
    }
}

impl Add for Point2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point2 {
    type Output = Vector2;

    fn sub(self, other: Self) -> Self::Output {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: Into<f64>> Mul<T> for Point2 {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        let other = other.into();
        Self::new(self.x * other, self.y * other)
    }
}

impl<T: Into<f64>> Div<T> for Point2 {
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        let other = other.into();
        Self::new(self.x / other, self.y / other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_equal() {
        let p1 = Point2 { x: 1.0, y: 1.0 };
        let p2 = Point2 { x: 1.0, y: 1.0 };
        let result = p1.is_equal(&p2, None);
        assert_eq!(result, true);

        let p1 = Point2 { x: 1.0, y: 1.0 };
        let p2 = Point2 { x: 1.0, y: 1.1 };
        let result = p1.is_equal(&p2, None);
        assert_eq!(result, false);

        let p1 = Point2 { x: 1.0, y: 1.0 };
        let p2 = Point2 { x: 1.1, y: 1.0 };
        let result = p1.is_equal(&p2, None);
        assert_eq!(result, false);

        let p1 = Point2 { x: 1.0, y: 1.0 };
        let p2 = Point2 { x: 1.1, y: 1.1 };
        let result = p1.is_equal(&p2, Some(Eps::new(0.01)));
        assert_eq!(result, false);
    }
}
