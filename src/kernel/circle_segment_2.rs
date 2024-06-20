use super::{
    arc_segment_2::ArcSegment2,
    number_type::NumberType,
    point_2::Point2,
    segment_2::Segment2,
    util_enum::{Orientation, Segment2Type},
};

#[derive(Debug, Clone, Copy)]
pub struct CircleSegment2<T: NumberType> {
    center: Point2<T>,
    radius: T,
}

impl<T: NumberType> CircleSegment2<T> {
    pub fn new(center: Point2<T>, radius: T) -> Self {
        Self { center, radius }
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

impl<T: NumberType> Segment2<T> for CircleSegment2<T> {
    fn segment_type(&self) -> Segment2Type {
        return Segment2Type::CircleSegment2;
    }

    fn source(&self) -> Point2<T> {
        panic!("CircleSegment2 does not have a source point")
    }

    fn target(&self) -> Point2<T> {
        panic!("CircleSegment2 does not have a target point")
    }

    fn center(&self) -> Point2<T> {
        self.center.clone()
    }

    fn radius(&self) -> T {
        self.radius
    }

    fn orientation(&self) -> Orientation {
        panic!("CircleSegment2 does not have an orientation")
    }

    fn reverse_orientation(&mut self) {
        panic!("CircleSegment2 does not have an orientation")
    }
}
