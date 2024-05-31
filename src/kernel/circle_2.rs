use super::{number_type::NumberType, point_2::Point2};

#[derive(Debug, Clone, Copy)]
pub struct Circle2<NT: NumberType> {
    center: Point2<NT>,
    radius: NT,
}

impl<NT: NumberType> Circle2<NT> {
    pub fn new(center: Point2<NT>, radius: NT) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> Point2<NT> {
        Point2::new(self.center.x(), self.center.y())
    }

    pub fn radius(&self) -> NT {
        self.radius
    }
}
