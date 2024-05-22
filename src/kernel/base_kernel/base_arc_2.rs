use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

use super::{base_circle_2::BaseCircle2, base_point_2::BasePoint2};

pub trait BaseArc2<NT: BaseNumberTypeTrait>: Copy + Clone + Sized {
    type Circle2: BaseCircle2<NT>;
    type Point2: BasePoint2<NT>;

    fn new(support: Self::Circle2, source: Self::Point2, target: Self::Point2) -> Self;
    fn center(&self) -> Self::Point2;
    fn radius(&self) -> NT;
    fn source(&self) -> Self::Point2;
    fn target(&self) -> Self::Point2;
}
