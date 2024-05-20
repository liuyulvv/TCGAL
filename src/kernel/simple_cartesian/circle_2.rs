use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

use super::point_2::Point2;

#[derive(Debug, Clone, Copy)]
pub struct Circle2<T: BaseNumberTypeTrait> {
    center: Point2<T>,
    radius: T,
}

impl<T: BaseNumberTypeTrait> Circle2<T> {
    pub fn new(center: Point2<T>, radius: T) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> Point2<T> {
        self.center
    }

    pub fn radius(&self) -> T {
        self.radius
    }
}
