use crate::{
    kernel::base_kernel::{base_point_2::BasePoint2, base_segment_2::BaseSegment2},
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

use super::point_2::Point2;

#[derive(Debug, Clone, Copy)]
pub struct Segment2<'a, T: BaseNumberTypeTrait> {
    source: &'a Point2<T>,
    target: &'a Point2<T>,
}

impl<'a, T: BaseNumberTypeTrait> Segment2<'a, T> {
    pub fn new(source: &'a Point2<T>, target: &'a Point2<T>) -> Self {
        Self { source, target }
    }
}

impl<'a, T: BaseNumberTypeTrait> BaseSegment2<'a, T> for Segment2<'a, T> {
    fn source(&self) -> Box<dyn BasePoint2<T> + 'a> {
        Box::new(*self.source)
    }

    fn target(&self) -> Box<dyn BasePoint2<T> + 'a> {
        Box::new(*self.target)
    }
}
