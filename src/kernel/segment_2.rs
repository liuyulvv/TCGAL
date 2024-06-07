use super::{number_type::NumberType, point_2::Point2};

#[derive(Debug, Clone, Copy)]
pub struct Segment2<T: NumberType> {
    source: Point2<T>,
    target: Point2<T>,
}

impl<T: NumberType> Segment2<T> {
    pub fn new(source: Point2<T>, target: Point2<T>) -> Self {
        Self { source, target }
    }

    pub fn source(&self) -> Point2<T> {
        Point2::new(self.source.x(), self.source.y())
    }

    pub fn target(&self) -> Point2<T> {
        Point2::new(self.target.x(), self.target.y())
    }
}
