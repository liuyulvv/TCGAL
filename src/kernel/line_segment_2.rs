use super::{number_type::NumberType, point_2::Point2};

#[derive(Debug, Clone, Copy)]
pub struct LineSegment2<T: NumberType> {
    source: Point2<T>,
    target: Point2<T>,
}

impl<T: NumberType> LineSegment2<T> {
    pub fn new(source: Point2<T>, target: Point2<T>) -> Self {
        Self { source, target }
    }

    pub fn source(&self) -> Point2<T> {
        self.source.clone()
    }

    pub fn target(&self) -> Point2<T> {
        self.target.clone()
    }
}

impl<T: NumberType> PartialEq for LineSegment2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.target == other.target
    }
}
