use crate::traits::{eps::Eps, is_equal::IsEqual, is_parallel::IsParallel, is_same::IsSame};

use super::point_2::Point2;

pub struct Segment2 {
    pub start: Point2,
    pub end: Point2,
}

impl Segment2 {
    pub fn new(start: Point2, end: Point2) -> Self {
        Self { start, end }
    }

    pub fn length(&self) -> f64 {
        self.start.distance_to(&self.end)
    }
}

impl IsParallel for Segment2 {
    fn is_parallel(&self, other: &Self, eps: Option<Eps>) -> bool {
        let eps = eps.unwrap_or(Eps::default()).value;
        let self_vector = self.end - self.start;
        let other_vector = other.end - other.start;
        self_vector.cross(&other_vector).abs() < eps
    }
}

impl IsEqual for Segment2 {
    fn is_equal(&self, other: &Self, eps: Option<Eps>) -> bool {
        self.start.is_equal(&other.start, eps) && self.end.is_equal(&other.end, eps)
    }
}

impl IsSame for Segment2 {
    fn is_same(&self, other: &Self, eps: Option<Eps>) -> bool {
        (self.start.is_same(&other.start, eps) && self.end.is_same(&other.end, eps))
            || (self.start.is_same(&other.end, eps) && self.end.is_same(&other.start, eps))
    }
}

impl PartialEq for Segment2 {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_equal() {
        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p1, p2);
        let result = s1.is_equal(&s2, None);
        assert_eq!(result, true);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p1, p2);
        let result = s1.is_equal(&s2, None);
        assert_eq!(result, true);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p2, p1);
        let result = s1.is_equal(&s2, None);
        assert_eq!(result, false);
    }

    #[test]
    fn is_same() {
        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p1, p2);
        let result = s1.is_same(&s2, None);
        assert_eq!(result, true);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p1, p2);
        let result = s1.is_same(&s2, None);
        assert_eq!(result, true);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p2, p1);
        let result = s1.is_same(&s2, None);
        assert_eq!(result, true);
    }

    #[test]
    fn is_parallel() {
        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p1, p2);
        let result = s1.is_parallel(&s2, None);
        assert_eq!(result, true);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p1, p2);
        let result = s1.is_parallel(&s2, None);
        assert_eq!(result, true);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p2, p1);
        let result = s1.is_parallel(&s2, None);
        assert_eq!(result, true);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let p3 = Point2::new(1.1, 1.2);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p2, p3);
        let result = s1.is_parallel(&s2, None);
        assert_eq!(result, false);
    }
}
