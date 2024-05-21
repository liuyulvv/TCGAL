use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

use super::base_point_2::BasePoint2;

pub trait BaseSegment2<'a, NT: BaseNumberTypeTrait>: Copy + Clone + Sized {
    type Point2: BasePoint2<NT>;

    fn new(source: &'a Self::Point2, target: &'a Self::Point2) -> Self;
    fn source(&self) -> Self::Point2;
    fn target(&self) -> Self::Point2;
}
