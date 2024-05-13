use crate::traits::eps::Eps;
use crate::traits::is_equal::IsEqual;
use crate::traits::is_parallel::IsParallel;
use crate::traits::is_same::IsSame;

use super::point_2::Point2;

#[derive(Debug)]
pub struct Line2 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Line2 {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }

    pub fn from_points(start: Point2, end: Point2) -> Self {
        let a = end.y - start.y;
        let b = start.x - end.x;
        let c = start.x * (start.y - end.y) + start.y * (end.x - start.x);
        Self { a, b, c }
    }
}

impl IsParallel for Line2 {
    fn is_parallel(&self, other: &Self, eps: Option<Eps>) -> bool {
        let eps = eps.unwrap_or(Eps::default()).value;
        let det = self.a * other.b - self.b * other.a;
        return det.abs() < eps;
    }
}

impl IsSame for Line2 {
    fn is_same(&self, other: &Self, eps: Option<Eps>) -> bool {
        let eps = eps.unwrap_or(Eps::default()).value;
        let det = self.a * other.b - self.b * other.a;
        let det_c = self.c * other.b - self.b * other.c;
        let det_a = self.a * other.c - self.c * other.a;
        return det.abs() < eps && det_c.abs() < eps && det_a.abs() < eps;
    }
}

impl IsEqual for Line2 {
    fn is_equal(&self, other: &Self, eps: Option<Eps>) -> bool {
        self.is_same(other, eps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_parallel() {
        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 1.0, 1.0);
        let result = l1.is_parallel(&l2, None);
        assert_eq!(result, true);

        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 1.0, 2.0);
        let result = l1.is_parallel(&l2, None);
        assert_eq!(result, true);

        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 2.0, 1.0);
        let result = l1.is_parallel(&l2, None);
        assert_eq!(result, false);

        let l1 = Line2::new(1.0, 0.0, 1.0);
        let l2 = Line2::new(1.0, 0.0, 2.0);
        let result = l1.is_parallel(&l2, None);
        assert_eq!(result, true);

        let l1 = Line2::new(0.0, 1.0, 1.0);
        let l2 = Line2::new(0.0, 1.0, 2.0);
        let result = l1.is_parallel(&l2, None);
        assert_eq!(result, true);

        let l1 = Line2::new(1.0, 0.0, 1.0);
        let l2 = Line2::new(0.0, 1.0, 2.0);
        let result = l1.is_parallel(&l2, None);
        assert_eq!(result, false);

        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 1.0001, 1.0);
        let result = l1.is_parallel(&l2, Some(Eps::new(0.0001)));
        assert_eq!(result, true);

        let result = l1.is_parallel(&l2, Some(Eps::new(0.00001)));
        assert_eq!(result, false);
    }
}
