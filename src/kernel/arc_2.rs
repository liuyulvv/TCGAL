use super::{circle_2::Circle2, number_type::NumberType, point_2::Point2, util_enum::Orientation};

#[derive(Debug, Clone, Copy)]
pub struct Arc2<T: NumberType> {
    support: Circle2<T>,
    source_radian: T,
    target_radian: T,
    orientation: Orientation,
}

impl<T: NumberType> Arc2<T> {
    pub fn new(support: Circle2<T>, source_radian: T, target_radian: T) -> Self {
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

    pub fn center(&self) -> Point2<T> {
        self.support.center()
    }

    pub fn radius(&self) -> T {
        self.support.radius()
    }

    pub fn source(&self) -> Point2<T> {
        let center = self.center();
        let radius = self.radius();
        let x = center.x() + radius * self.source_radian.cos();
        let y = center.y() + radius * self.source_radian.sin();
        Point2::new(x, y)
    }

    pub fn target(&self) -> Point2<T> {
        let center = self.center();
        let radius = self.radius();
        let x = center.x() + radius * self.target_radian.cos();
        let y = center.y() + radius * self.target_radian.sin();
        Point2::new(x, y)
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn reverse_orientation(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Clockwise => Orientation::CounterClockwise,
            Orientation::CounterClockwise => Orientation::Clockwise,
        };
    }

    pub fn monotone(&self) -> Vec<Arc2<T>> {
        let mut arcs = Vec::new();
        let pi = T::pi();
        match self.orientation {
            Orientation::CounterClockwise => {
                if self.target_radian > pi && self.source_radian < pi {
                    arcs.push(Arc2::new(self.support, self.source_radian, pi));
                    arcs.push(Arc2::new(self.support, pi, self.target_radian));
                } else {
                    arcs.push(Arc2::new(
                        self.support,
                        self.source_radian,
                        self.target_radian,
                    ));
                }
            }
            Orientation::Clockwise => {
                if self.source_radian > pi && self.target_radian < pi {
                    arcs.push(Arc2::new(self.support, pi, self.target_radian));
                    arcs.push(Arc2::new(self.support, self.source_radian, pi));
                } else {
                    arcs.push(Arc2::new(
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

impl<T: NumberType> PartialEq for Arc2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.center() == other.center()
            && self.radius() == other.radius()
            && self.source_radian.equals(other.source_radian)
            && self.target_radian.equals(other.target_radian)
    }
}
