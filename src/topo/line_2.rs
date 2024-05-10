use crate::traits::eps::Eps;
use crate::traits::is_parallel::IsParallel;

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

    pub fn is_intersect(&self, other: &Self, eps: Option<Eps>) -> bool {
        let eps = eps.unwrap_or(Eps::default()).value;
        let det = self.a * other.b - self.b * other.a;
        if det.abs() < eps {
            return false;
        }
        let x = (self.b * other.c - self.c * other.b) / det;
        let y = (self.c * other.a - self.a * other.c) / det;
        let is_self_on_line = self.a * x + self.b * y - self.c < eps;
        let is_other_on_line = other.a * x + other.b * y - other.c < eps;
        return is_self_on_line && is_other_on_line;
    }

    pub fn intersection(&self, other: &Self, eps: Option<Eps>) -> Option<Point2> {
        let eps = eps.unwrap_or(Eps::default()).value;
        let det = self.a * other.b - self.b * other.a;
        if det.abs() < eps {
            return None;
        }
        let x = (self.b * other.c - self.c * other.b) / det;
        let y = (self.c * other.a - self.a * other.c) / det;
        let is_self_on_line = self.a * x + self.b * y - self.c < eps;
        let is_other_on_line = other.a * x + other.b * y - other.c < eps;
        if is_self_on_line && is_other_on_line {
            return Some(Point2::new(x, y));
        }
        return None;
    }
}

impl IsParallel for Line2 {
    fn is_parallel(&self, other: &Self, eps: Option<Eps>) -> bool {
        let eps = eps.unwrap_or(Eps::default()).value;
        let det = self.a * other.b - self.b * other.a;
        return det.abs() < eps;
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

        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 2.0, 2.0);
        let result = l1.is_parallel(&l2, None);
        assert_eq!(result, false);

        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 1.0, 1.0);
        let result = l1.is_parallel(&l2, Some(Eps::new(0.2)));
        assert_eq!(result, true);

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

    #[test]
    fn is_intersect() {
        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 1.0, 1.0);
        let result = l1.is_intersect(&l2, None);
        assert_eq!(result, false);

        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 1.0, 2.0);
        let result = l1.is_intersect(&l2, None);
        assert_eq!(result, false);

        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 2.0, 1.0);
        let result = l1.is_intersect(&l2, None);
        assert_eq!(result, true);

        let l1 = Line2::new(1.0, 0.0, 1.0);
        let l2 = Line2::new(1.0, 0.0, 2.0);
        let result = l1.is_intersect(&l2, None);
        assert_eq!(result, false);

        let l1 = Line2::new(0.0, 1.0, 1.0);
        let l2 = Line2::new(0.0, 1.0, 2.0);
        let result = l1.is_intersect(&l2, None);
        assert_eq!(result, false);
    }

    #[test]
    fn intersection() {
        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 1.0, 1.0);
        let result = l1.intersection(&l2, None);
        assert_eq!(result, None);

        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 1.0, 2.0);
        let result = l1.intersection(&l2, None);
        assert_eq!(result, None);

        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 2.0, 1.0);
        let result = l1.intersection(&l2, None);
        assert_eq!(result, Some(Point2::new(-1.0, 0.0)));

        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 2.0, 2.0);
        let result = l1.intersection(&l2, None);
        assert_eq!(result, Some(Point2::new(0.0, -1.0)));

        let l1 = Line2::new(1.0, 0.0, 1.0);
        let l2 = Line2::new(1.0, 0.0, 2.0);
        let result = l1.intersection(&l2, None);
        assert_eq!(result, None);
    }
}
