use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

use super::base_point_2::BasePoint2;

pub trait BaseCircle2<NT: BaseNumberTypeTrait>: Copy + Clone + Sized {
    type Point2: BasePoint2<NT>;

    fn new(center: Self::Point2, radius: NT) -> Self;
    fn center(&self) -> Self::Point2;
    fn radius(&self) -> NT;
}
