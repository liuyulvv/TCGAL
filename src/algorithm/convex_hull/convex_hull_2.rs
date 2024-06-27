use crate::kernel::{number_type::NumberType, point_2::Point2};

pub struct ConvexHull2<T: NumberType> {
    pub points: Vec<Point2<T>>,
}

impl<T: NumberType> ConvexHull2<T> {
    pub fn new(points: Vec<Point2<T>>) -> Self {
        Self { points }
    }

    pub fn convex_hull(&mut self) -> Vec<Point2<T>> {
        self.points.sort_by(|a, b| {
            if !a.x().equals(b.x()) {
                a.x().partial_cmp(&b.x()).unwrap()
            } else {
                a.y().partial_cmp(&b.y()).unwrap()
            }
        });
        let mut lower: Vec<Point2<T>> = vec![];
        let mut upper: Vec<Point2<T>> = vec![];
        for p in self.points.iter() {
            while lower.len() >= 2
                && Self::cross(&lower[lower.len() - 2], &lower[lower.len() - 1], &p) <= T::zero()
            {
                lower.pop();
            }
            lower.push(p.clone());
        }
        for p in self.points.iter().rev() {
            while upper.len() >= 2
                && Self::cross(&upper[upper.len() - 2], &upper[upper.len() - 1], &p) <= T::zero()
            {
                upper.pop();
            }
            upper.push(p.clone());
        }
        lower.pop();
        upper.pop();
        lower.extend(upper);
        lower
    }

    fn cross(a: &Point2<T>, b: &Point2<T>, c: &Point2<T>) -> T {
        (b.x() - a.x()) * (c.y() - a.y()) - (b.y() - a.y()) * (c.x() - a.x())
    }
}
