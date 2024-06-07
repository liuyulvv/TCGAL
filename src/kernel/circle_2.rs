use super::{number_type::NumberType, point_2::Point2};

#[derive(Debug, Clone, Copy)]
pub struct Circle2<T: NumberType> {
    center: Point2<T>,
    radius: T,
}

impl<T: NumberType> Circle2<T> {
    pub fn new(center: Point2<T>, radius: T) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> Point2<T> {
        Point2::new(self.center.x(), self.center.y())
    }

    pub fn radius(&self) -> T {
        self.radius
    }
}
