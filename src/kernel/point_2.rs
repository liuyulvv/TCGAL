use std::ops::{Add, Sub};

use super::{number_type::NumberType, vector_2::Vector2};

#[derive(Debug, Clone, Copy)]
pub struct Point2<NT: NumberType> {
    x: NT,
    y: NT,
}

impl<NT: NumberType> Point2<NT> {
    pub fn new(x: NT, y: NT) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> NT {
        self.x
    }

    pub fn y(&self) -> NT {
        self.y
    }

    pub fn equals(&self, other: &Self) -> bool {
        let eps = NT::default_eps();
        (self.x - other.x).abs() < eps && (self.y - other.y).abs() < eps
    }

    pub fn get_vector(&self) -> Vector2<NT> {
        Vector2::new(self.x, self.y)
    }
}

impl<NT: NumberType> Add for Point2<NT> {
    type Output = Vector2<NT>;

    fn add(self, other: Self) -> Self::Output {
        Vector2::new(self.x + other.x(), self.y + other.y())
    }
}

impl<NT: NumberType> Sub for Point2<NT> {
    type Output = Vector2<NT>;

    fn sub(self, other: Self) -> Self::Output {
        Vector2::new(self.x - other.x(), self.y - other.y())
    }
}

impl<NT: NumberType> PartialEq for Point2<NT> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<NT: NumberType> PartialOrd for Point2<NT> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.equals(other) {
            return Some(std::cmp::Ordering::Equal);
        }
        if self.x < other.x {
            Some(std::cmp::Ordering::Less)
        } else if self.x > other.x {
            Some(std::cmp::Ordering::Greater)
        } else if self.y < other.y {
            Some(std::cmp::Ordering::Less)
        } else if self.y > other.y {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}
