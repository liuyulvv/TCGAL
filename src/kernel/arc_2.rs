use super::{circle_2::Circle2, number_type::NumberType, point_2::Point2};

#[derive(Debug, Clone, Copy)]
pub struct Arc2<T: NumberType> {
    support: Circle2<T>,
    source: Point2<T>,
    target: Point2<T>,
}

impl<T: NumberType> Arc2<T> {
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
        Point2::new(self.source.x(), self.source.y())
    }

    pub fn target(&self) -> Point2<T> {
        Point2::new(self.target.x(), self.target.y())
    }
}
