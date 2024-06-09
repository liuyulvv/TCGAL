use std::ops::{Add, Sub};

use super::{number_type::NumberType, vector_2::Vector2};

#[derive(Debug, Clone, Copy)]
pub struct Point2<T: NumberType> {
    x: T,
    y: T,
}

impl<T: NumberType> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn equals(&self, other: &Self) -> bool {
        let eps = T::default_eps();
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)).sqrt()
            < eps
    }

    pub fn get_vector(&self) -> Vector2<T> {
        Vector2::new(self.x, self.y)
    }

    pub fn distance(&self, other: &Self) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

impl<T: NumberType> Add for Point2<T> {
    type Output = Vector2<T>;

    fn add(self, other: Self) -> Self::Output {
        Vector2::new(self.x + other.x(), self.y + other.y())
    }
}

impl<T: NumberType> Sub for Point2<T> {
    type Output = Vector2<T>;

    fn sub(self, other: Self) -> Self::Output {
        Vector2::new(self.x - other.x(), self.y - other.y())
    }
}

impl<T: NumberType> Eq for Point2<T> {}

impl<T: NumberType> PartialEq for Point2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl<T: NumberType> Ord for Point2<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.equals(other) {
            return std::cmp::Ordering::Equal;
        }
        if self.y > other.y {
            return std::cmp::Ordering::Greater;
        } else if self.y < other.y {
            return std::cmp::Ordering::Less;
        } else {
            if self.x > other.x {
                return std::cmp::Ordering::Less;
            } else if self.x < other.x {
                return std::cmp::Ordering::Greater;
            } else {
                return std::cmp::Ordering::Equal;
            }
        }
    }
}

impl<T: NumberType> PartialOrd for Point2<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
