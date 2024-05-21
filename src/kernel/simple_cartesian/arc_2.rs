use crate::{
    kernel::base_kernel::{
        base_arc_2::BaseArc2, base_circle_2::BaseCircle2, base_point_2::BasePoint2,
    },
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

use super::{circle_2::Circle2, point_2::Point2};

#[derive(Debug, Clone, Copy)]
pub struct Arc2<'a, NT: BaseNumberTypeTrait> {
    support: &'a Circle2<'a, NT>,
    source: &'a Point2<NT>,
    target: &'a Point2<NT>,
}

impl<'a, NT: BaseNumberTypeTrait> Arc2<'a, NT> {
    pub fn new(support: &'a Circle2<NT>, source: &'a Point2<NT>, target: &'a Point2<NT>) -> Self {
        Self {
            support,
            source,
            target,
        }
    }
}

impl<'a, NT: BaseNumberTypeTrait> BaseArc2<'a, NT> for Arc2<'a, NT> {
    type Circle2 = Circle2<'a, NT>;
    type Point2 = Point2<NT>;

    fn new(support: &'a Circle2<NT>, source: &'a Self::Point2, target: &'a Self::Point2) -> Self {
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
