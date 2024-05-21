use std::ops::{Add, Div, Mul, Sub};

use crate::{
    kernel::base_kernel::base_vector_2::BaseVector2,
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

#[derive(Debug, Clone, Copy)]
pub struct Vector2<NT: BaseNumberTypeTrait> {
    x: NT,
    y: NT,
}

impl<NT: BaseNumberTypeTrait> BaseVector2<NT> for Vector2<NT> {
    fn new(x: NT, y: NT) -> Self {
        Self { x, y }
    }

    fn x(&self) -> NT {
        self.x
    }

    fn y(&self) -> NT {
        self.y
    }

    fn length(&self) -> NT {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(&self) -> Self {
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

    fn dot(&self, other: &Self) -> NT {
        self.x * other.x + self.y * other.y
    }

    fn cross(&self, other: &Self) -> NT {
        self.x * other.y - self.y * other.x
    }
}

impl<NT: BaseNumberTypeTrait> Add for Vector2<NT> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl<NT: BaseNumberTypeTrait> Sub for Vector2<NT> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<NT: BaseNumberTypeTrait> Mul<NT> for Vector2<NT> {
    type Output = Self;

    fn mul(self, rhs: NT) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<NT: BaseNumberTypeTrait> Div<NT> for Vector2<NT> {
    type Output = Self;

    fn div(self, rhs: NT) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl<NT: BaseNumberTypeTrait> PartialEq for Vector2<NT> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
