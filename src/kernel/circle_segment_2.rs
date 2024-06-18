use super::{arc_segment_2::ArcSegment2, number_type::NumberType, point_2::Point2};

#[derive(Debug, Clone, Copy)]
pub struct CircleSegment2<T: NumberType> {
    center: Point2<T>,
    radius: T,
}

impl<T: NumberType> CircleSegment2<T> {
    pub fn new(center: Point2<T>, radius: T) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> Point2<T> {
        Point2::new(self.center.x(), self.center.y())
    }

    pub fn radius(&self) -> T {
        self.radius
    }

    pub fn monotone(&self) -> Vec<ArcSegment2<T>> {
        let mut arcs = Vec::new();
        let pi = T::pi();
        let two_pi = pi * T::from_f64(2.0);
        arcs.push(ArcSegment2::new(self.clone(), T::zero(), pi));
        arcs.push(ArcSegment2::new(self.clone(), pi, two_pi));
        arcs
    }
}
