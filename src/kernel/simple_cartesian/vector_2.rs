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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_vector_2() {
        let vector_2_f32 = Vector2::new(1.0f32, 2.0f32);
        assert_eq!(vector_2_f32.length(), 5.0f32.sqrt());

        let vector_2_f64 = Vector2::new(1.0f64, 2.0f64);
        assert_eq!(vector_2_f64.length(), 5.0f64.sqrt());

        let vector_2_f64_a = Vector2::new(1.0, 2.0);
        let vector_2_f64_b = Vector2::new(3.0, 4.0);
        let vector_2_f64_add = vector_2_f64_a + vector_2_f64_b;
        let vector_2_f64_sub = vector_2_f64_a - vector_2_f64_b;
        let vector_2_f64_mul = vector_2_f64_a * 2.0;
        assert_eq!(vector_2_f64_add, Vector2::new(4.0, 6.0));
        assert_eq!(vector_2_f64_sub, Vector2::new(-2.0, -2.0));
        assert_eq!(vector_2_f64_mul, Vector2::new(2.0, 4.0));
    }
}
