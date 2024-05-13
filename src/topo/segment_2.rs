use crate::traits::{eps::Eps, is_equal::IsEqual, is_parallel::IsParallel, is_same::IsSame};

use super::point_2::Point2;

pub struct Segment2 {
    start: Point2,
    end: Point2,
}

impl Segment2 {
    pub fn new(start: Point2, end: Point2) -> Self {
        Self { start, end }
    }

    pub fn get_start(&self) -> &Point2 {
        &self.start
    }

    pub fn get_end(&self) -> &Point2 {
        &self.end
    }

    pub fn set_start(&mut self, start: Point2) {
        self.start = start;
    }

    pub fn set_end(&mut self, end: Point2) {
        self.end = end;
    }

    pub fn length(&self) -> f64 {
        self.start.distance_to(&self.end)
    }

    pub fn is_intersect(&self, other: &Self, eps: Option<Eps>) -> bool {
        let eps = eps.unwrap_or(Eps::default()).value;
        let ab = self.end - self.start;
        let ac = other.start - self.start;
        let ad = other.end - self.start;
        let cd = other.end - other.start;
        let ca = self.start - other.start;
        let cb = self.end - other.start;
        ab.cross(&ac) * ab.cross(&ad) < eps && cd.cross(&ca) * cd.cross(&cb) < eps
    }

    /// TODO: implement overlap
    /// overlay is not implemented
    pub fn intersection(&self, other: &Self, eps: Option<Eps>) -> Option<Point2> {
        if !self.is_intersect(other, eps) {
            return None;
        }
        let eps = eps.unwrap_or(Eps::default()).value;
        let ab = self.end - self.start;
        let cd = other.end - other.start;
        let ac = other.start - self.start;
        let cross_ab_cd = ab.cross(&cd);
        if cross_ab_cd.abs() < eps || cross_ab_cd.is_nan() {
            return None;
        }
        let t = ac.cross(&cd) / ab.cross(&cd);
        let res = self.start.get_vector() + ab * t;
        return Some(Point2::from_vector(&res));
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
        self.start.is_equal(other.get_start(), eps) && self.end.is_equal(other.get_end(), eps)
    }
}

impl IsSame for Segment2 {
    fn is_same(&self, other: &Self, eps: Option<Eps>) -> bool {
        (self.start.is_same(other.get_start(), eps) && self.end.is_same(other.get_end(), eps))
            || (self.start.is_same(other.get_end(), eps)
                && self.end.is_same(other.get_start(), eps))
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

    #[test]
    fn is_intersect() {
        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p1, p2);
        let result = s1.is_intersect(&s2, None);
        assert_eq!(result, true);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p1, p2);
        let result = s1.is_intersect(&s2, None);
        assert_eq!(result, true);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p2, p1);
        let result = s1.is_intersect(&s2, None);
        assert_eq!(result, true);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let p3 = Point2::new(1.1, 1.2);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p2, p3);
        let result = s1.is_intersect(&s2, None);
        assert_eq!(result, true);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let p3 = Point2::new(1.1, 1.2);
        let p4 = Point2::new(1.2, 1.3);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p3, p4);
        let result = s1.is_intersect(&s2, None);
        assert_eq!(result, false);
    }

    #[test]
    fn intersection() {
        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p1, p2);
        let result = s1.intersection(&s2, None);
        assert_eq!(result, None);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 2.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p1, p2);
        let result = s1.intersection(&s2, None);
        assert_eq!(result, None);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let p3 = Point2::new(1.1, 1.2);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p2, p3);
        let result = s1.intersection(&s2, None);
        assert_eq!(result, Some(Point2::new(1.0, 1.1)));

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let p3 = Point2::new(1.1, 1.2);
        let p4 = Point2::new(1.2, 1.3);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p3, p4);
        let result = s1.intersection(&s2, None);
        assert_eq!(result, None);

        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let p3 = Point2::new(0.0, 5.0);
        let p4 = Point2::new(10.0, 5.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p3, p4);
        let result = s1.intersection(&s2, None);
        assert_eq!(result, Some(Point2::new(5.0, 5.0)));

        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let p3 = Point2::new(0.0, 5.0);
        let p4 = Point2::new(10.0, 0.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p3, p4);
        let result = s1.intersection(&s2, None);
        assert_eq!(result, Some(Point2::new(3.333333, 3.333333)));
    }
}
