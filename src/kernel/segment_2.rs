use super::{number_type::NumberType, point_2::Point2};

#[derive(Debug, Clone, Copy)]
pub struct Segment2<NT: NumberType> {
    source: Point2<NT>,
    target: Point2<NT>,
}

impl<NT: NumberType> Segment2<NT> {
    pub fn new(source: Point2<NT>, target: Point2<NT>) -> Self {
        Self { source, target }
    }

    pub fn source(&self) -> Point2<NT> {
        Point2::new(self.source.x(), self.source.y())
    }

    pub fn target(&self) -> Point2<NT> {
        Point2::new(self.target.x(), self.target.y())
    }
}
