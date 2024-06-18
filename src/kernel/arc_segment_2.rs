use super::{
    circle_segment_2::CircleSegment2,
    number_type::NumberType,
    point_2::Point2,
    segment_2::Segment2,
    util_enum::{Orientation, Segment2Type},
};

#[derive(Debug, Clone, Copy)]
pub struct ArcSegment2<T: NumberType> {
    support: CircleSegment2<T>,
    source_radian: T,
    target_radian: T,
    orientation: Orientation,
}

impl<T: NumberType> ArcSegment2<T> {
    pub fn new(support: CircleSegment2<T>, source_radian: T, target_radian: T) -> Self {
        let mut source_radian = source_radian;
        let mut target_radian = target_radian;
        let pi = T::pi();
        let two_pi = pi * T::from_f64(2.0);
        while !source_radian.equals(two_pi) && source_radian > two_pi {
            source_radian = source_radian - two_pi;
        }
        while !target_radian.equals(two_pi) && target_radian > two_pi {
            target_radian = target_radian - two_pi;
        }
        if source_radian.equals(target_radian) {
            source_radian = T::zero();
            target_radian = two_pi;
        }
        let mut orientation = Orientation::CounterClockwise;
        if source_radian > target_radian {
            orientation = Orientation::Clockwise;
        }
        Self {
            support,
            source_radian,
            target_radian,
            orientation,
        }
    }

    pub fn monotone(&self) -> Vec<ArcSegment2<T>> {
        let mut arcs = Vec::new();
        let pi = T::pi();
        match self.orientation {
            Orientation::CounterClockwise => {
                if self.target_radian > pi && self.source_radian < pi {
                    arcs.push(ArcSegment2::new(self.support, self.source_radian, pi));
                    arcs.push(ArcSegment2::new(self.support, pi, self.target_radian));
                } else {
                    arcs.push(ArcSegment2::new(
                        self.support,
                        self.source_radian,
                        self.target_radian,
                    ));
                }
            }
            Orientation::Clockwise => {
                if self.source_radian > pi && self.target_radian < pi {
                    arcs.push(ArcSegment2::new(self.support, pi, self.target_radian));
                    arcs.push(ArcSegment2::new(self.support, self.source_radian, pi));
                } else {
                    arcs.push(ArcSegment2::new(
                        self.support,
                        self.target_radian,
                        self.source_radian,
                    ));
                }
            }
        }
        arcs
    }
}

impl<T: NumberType> Segment2<T> for ArcSegment2<T> {
    fn segment_type(&self) -> Segment2Type {
        Segment2Type::ArcSegment2
    }

    fn source(&self) -> Point2<T> {
        let center = self.center();
        let radius = self.radius();
        let x = center.x() + radius * self.source_radian.cos();
        let y = center.y() + radius * self.source_radian.sin();
        Point2::new(x, y)
    }

    fn target(&self) -> Point2<T> {
        let center = self.center();
        let radius = self.radius();
        let x = center.x() + radius * self.target_radian.cos();
        let y = center.y() + radius * self.target_radian.sin();
        Point2::new(x, y)
    }

    fn center(&self) -> Point2<T> {
        self.support.center()
    }

    fn radius(&self) -> T {
        self.support.radius()
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn reverse_orientation(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Clockwise => Orientation::CounterClockwise,
            Orientation::CounterClockwise => Orientation::Clockwise,
        };
    }
}

impl<T: NumberType> PartialEq for ArcSegment2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.center() == other.center()
            && self.radius() == other.radius()
            && self.source_radian.equals(other.source_radian)
            && self.target_radian.equals(other.target_radian)
    }
}
