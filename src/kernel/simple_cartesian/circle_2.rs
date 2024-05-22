use crate::{
    kernel::base_kernel::{base_circle_2::BaseCircle2, base_point_2::BasePoint2},
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

use super::point_2::Point2;

#[derive(Debug, Clone, Copy)]
pub struct Circle2<NT: BaseNumberTypeTrait> {
    center: Point2<NT>,
    radius: NT,
}

impl<NT: BaseNumberTypeTrait> BaseCircle2<NT> for Circle2<NT> {
    type Point2 = Point2<NT>;

    fn new(center: Self::Point2, radius: NT) -> Self {
        Self { center, radius }
    }

    fn center(&self) -> Self::Point2 {
        Self::Point2::new(self.center.x(), self.center.y())
    }

    fn radius(&self) -> NT {
        self.radius
    }
}
