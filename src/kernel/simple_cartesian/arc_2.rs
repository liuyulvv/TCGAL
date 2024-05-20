use crate::{
    kernel::base_kernel::{
        base_arc_2::BaseArc2, base_circle_2::BaseCircle2, base_point_2::BasePoint2,
    },
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

use super::{circle_2::Circle2, point_2::Point2};

pub struct Arc2<'a, T: BaseNumberTypeTrait> {
    support: &'a Circle2<'a, T>,
    source: &'a Point2<T>,
    target: &'a Point2<T>,
}

impl<'a, T: BaseNumberTypeTrait> Arc2<'a, T> {
    pub fn new(support: &'a Circle2<T>, source: &'a Point2<T>, target: &'a Point2<T>) -> Self {
        Self {
            support,
            source,
            target,
        }
    }
}

impl<'a, T: BaseNumberTypeTrait> BaseArc2<'a, T> for Arc2<'a, T> {
    fn center(&self) -> Box<dyn BasePoint2<T> + 'a> {
        self.support.center()
    }

    fn radius(&self) -> T {
        self.support.radius()
    }

    fn source(&self) -> Box<dyn BasePoint2<T> + 'a> {
        Box::new(*self.source)
    }

    fn target(&self) -> Box<dyn BasePoint2<T> + 'a> {
        Box::new(*self.target)
    }
}
