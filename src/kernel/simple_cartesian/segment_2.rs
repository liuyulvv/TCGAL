use crate::{
    kernel::base_kernel::{base_point_2::BasePoint2, base_segment_2::BaseSegment2},
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

use super::point_2::Point2;

#[derive(Debug, Clone, Copy)]
pub struct Segment2<NT: BaseNumberTypeTrait> {
    source: Point2<NT>,
    target: Point2<NT>,
}

impl<NT: BaseNumberTypeTrait> BaseSegment2<NT> for Segment2<NT> {
    type Point2 = Point2<NT>;

    fn new(source: Point2<NT>, target: Point2<NT>) -> Self {
        Self { source, target }
    }

    fn source(&self) -> Self::Point2 {
        Self::Point2::new(self.source.x(), self.source.y())
    }

    fn target(&self) -> Self::Point2 {
        Self::Point2::new(self.target.x(), self.target.y())
    }
}
