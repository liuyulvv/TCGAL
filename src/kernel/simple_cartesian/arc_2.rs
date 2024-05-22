use crate::{
    kernel::base_kernel::{
        base_arc_2::BaseArc2, base_circle_2::BaseCircle2, base_point_2::BasePoint2,
    },
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

use super::{circle_2::Circle2, point_2::Point2};

#[derive(Debug, Clone, Copy)]
pub struct Arc2<NT: BaseNumberTypeTrait> {
    support: Circle2<NT>,
    source: Point2<NT>,
    target: Point2<NT>,
}

impl<NT: BaseNumberTypeTrait> Arc2<NT> {
    pub fn new(support: Circle2<NT>, source: Point2<NT>, target: Point2<NT>) -> Self {
        Self {
            support,
            source,
            target,
        }
    }
}

impl<NT: BaseNumberTypeTrait> BaseArc2<NT> for Arc2<NT> {
    type Circle2 = Circle2<NT>;
    type Point2 = Point2<NT>;

    fn new(support: Circle2<NT>, source: Self::Point2, target: Self::Point2) -> Self {
        Self {
            support,
            source,
            target,
        }
    }

    fn center(&self) -> Self::Point2 {
        self.support.center()
    }

    fn radius(&self) -> NT {
        self.support.radius()
    }

    fn source(&self) -> Self::Point2 {
        Self::Point2::new(self.source.x(), self.source.y())
    }

    fn target(&self) -> Self::Point2 {
        Self::Point2::new(self.target.x(), self.target.y())
    }
}
