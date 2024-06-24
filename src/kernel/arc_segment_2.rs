use super::{
    circle_segment_2::CircleSegment2, number_type::NumberType, point_2::Point2,
    segment_2::Segment2, util_enum::Segment2Type,
};

#[derive(Debug, Clone, Copy)]
pub struct ArcSegment2<T: NumberType> {
    support: CircleSegment2<T>,
    source_radian: T,
    target_radian: T,
}

impl<T: NumberType> ArcSegment2<T> {
    pub fn new(support: CircleSegment2<T>, source_radian: T, target_radian: T) -> Self {
        let mut source_radian = source_radian;
        let mut target_radian = target_radian;
        let four_pi = T::pi() * T::from_f64(4.0);
        while !source_radian.equals(four_pi) && source_radian > four_pi {
            source_radian = source_radian - four_pi;
        }
        while !target_radian.equals(four_pi) && target_radian > four_pi {
            target_radian = target_radian - four_pi;
        }
        if source_radian.equals(target_radian) {
            source_radian = T::zero();
            target_radian = T::pi() * T::from_f64(2.0);
        }
        Self {
            support,
            source_radian,
            target_radian,
        }
    }

    pub fn is_top(&self) -> bool {
        let pi = T::pi();
        self.source_radian < pi && (self.target_radian.equals(pi) || self.target_radian < pi)
    }

    pub fn monotone(&self) -> Vec<ArcSegment2<T>> {
        let mut arcs = Vec::new();
        let pi = T::pi();
        let mut radians = Vec::new();
        let mut end_pi = T::zero();
        let mut min_flag = false;
        while !end_pi.equals(self.target_radian) && end_pi < self.target_radian {
            if !min_flag && end_pi > self.source_radian {
                min_flag = true;
                if end_pi > self.source_radian {
                    radians.push(end_pi - pi);
                    if !self.source_radian.equals(end_pi - pi) {
                        radians.push(self.source_radian.clone());
                    }
                }
            }
            if end_pi.equals(self.source_radian) || end_pi > self.source_radian {
                radians.push(end_pi.clone());
            }
            end_pi = end_pi + pi;
        }
        radians.push(self.target_radian.clone());
        for i in 0..radians.len() - 1 {
            arcs.push(ArcSegment2::new(
                self.support,
                radians[i].clone(),
                radians[i + 1].clone(),
            ));
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

    fn source_radian(&self) -> T {
        self.source_radian
    }

    fn target(&self) -> Point2<T> {
        let center = self.center();
        let radius = self.radius();
        let x = center.x() + radius * self.target_radian.cos();
        let y = center.y() + radius * self.target_radian.sin();
        Point2::new(x, y)
    }

    fn target_radian(&self) -> T {
        self.target_radian
    }

    fn center(&self) -> Point2<T> {
        self.support.center()
    }

    fn radius(&self) -> T {
        self.support.radius()
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
