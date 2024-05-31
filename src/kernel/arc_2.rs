use super::{circle_2::Circle2, number_type::NumberType, point_2::Point2};

#[derive(Debug, Clone, Copy)]
pub struct Arc2<NT: NumberType> {
    support: Circle2<NT>,
    source: Point2<NT>,
    target: Point2<NT>,
}

impl<NT: NumberType> Arc2<NT> {
    pub fn new(support: Circle2<NT>, source: Point2<NT>, target: Point2<NT>) -> Self {
        Self {
            support,
            source,
            target,
        }
    }

    pub fn center(&self) -> Point2<NT> {
        self.support.center()
    }

    pub fn radius(&self) -> NT {
        self.support.radius()
    }

    pub fn source(&self) -> Point2<NT> {
        Point2::new(self.source.x(), self.source.y())
    }

    pub fn target(&self) -> Point2<NT> {
        Point2::new(self.target.x(), self.target.y())
    }
}
