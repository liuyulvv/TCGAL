use std::ops::{Add, Sub};

use crate::{
    kernel::base_kernel::{base_point_2::BasePoint2, base_vector_2::BaseVector2},
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

use super::vector_2::Vector2;

#[derive(Debug, Clone, Copy)]
pub struct Point2<NT: BaseNumberTypeTrait> {
    x: NT,
    y: NT,
}

impl<NT: BaseNumberTypeTrait> BasePoint2<NT> for Point2<NT> {
    type Vector2 = Vector2<NT>;

    fn new(x: NT, y: NT) -> Self {
        Self { x, y }
    }

    fn x(&self) -> NT {
        self.x
    }

    fn y(&self) -> NT {
        self.y
    }

    fn equals(&self, other: &Self) -> bool {
        let eps = NT::default_eps();
        (self.x - other.x).abs() < eps && (self.y - other.y).abs() < eps
    }

    fn get_vector(&self) -> Self::Vector2 {
        Vector2::new(self.x, self.y)
    }
}

impl<NT: BaseNumberTypeTrait> Add for Point2<NT> {
    type Output = Vector2<NT>;

    fn add(self, other: Self) -> Self::Output {
        Vector2::new(self.x + other.x(), self.y + other.y())
    }
}

impl<NT: BaseNumberTypeTrait> Sub for Point2<NT> {
    type Output = Vector2<NT>;

    fn sub(self, other: Self) -> Self::Output {
        Vector2::new(self.x - other.x(), self.y - other.y())
    }
}

impl<NT: BaseNumberTypeTrait> PartialEq for Point2<NT> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<NT: BaseNumberTypeTrait> PartialOrd for Point2<NT> {
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
