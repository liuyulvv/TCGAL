use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

use super::{base_circle_2::BaseCircle2, base_point_2::BasePoint2};

pub trait BaseArc2<'a, NT: BaseNumberTypeTrait>: Copy + Clone + Sized {
    type Circle2: BaseCircle2<'a, NT>;
    type Point2: BasePoint2<NT>;

    fn new(support: &'a Self::Circle2, source: &'a Self::Point2, target: &'a Self::Point2) -> Self;
    fn center(&self) -> Self::Point2;
    fn radius(&self) -> NT;
    fn source(&self) -> Self::Point2;
    fn target(&self) -> Self::Point2;
}
