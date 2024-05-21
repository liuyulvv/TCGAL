use crate::{
    kernel::base_kernel::{base_circle_2::BaseCircle2, base_point_2::BasePoint2},
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

use super::point_2::Point2;

#[derive(Debug, Clone, Copy)]
pub struct Circle2<'a, NT: BaseNumberTypeTrait> {
    center: &'a Point2<NT>,
    radius: NT,
}

impl<'a, NT: BaseNumberTypeTrait> BaseCircle2<'a, NT> for Circle2<'a, NT> {
    type Point2 = Point2<NT>;

    fn new(center: &'a Self::Point2, radius: NT) -> Self {
        Self { center, radius }
    }

    fn center(&self) -> Self::Point2 {
        Self::Point2::new(self.center.x(), self.center.y())
    }

    fn radius(&self) -> NT {
        self.radius
    }
}
