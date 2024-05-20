use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

use super::{circle_2::Circle2, point_2::Point2};

pub struct Arc2<T: BaseNumberTypeTrait> {
    support: Circle2<T>,
    source: Point2<T>,
    target: Point2<T>,
}

impl<T: BaseNumberTypeTrait> Arc2<T> {
    pub fn new(support: Circle2<T>, source: Point2<T>, target: Point2<T>) -> Self {
        Self {
            support,
            source,
            target,
        }
    }

    pub fn center(&self) -> Point2<T> {
        self.support.center()
    }

    pub fn radius(&self) -> T {
        self.support.radius()
    }

    pub fn source(&self) -> Point2<T> {
        return self.source;
    }

    pub fn target(&self) -> Point2<T> {
        return self.target;
    }
}
