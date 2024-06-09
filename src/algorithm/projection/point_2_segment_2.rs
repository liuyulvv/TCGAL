use crate::kernel::{number_type::NumberType, point_2::Point2, segment_2::Segment2};

pub fn point_2_project_segment_2<T: NumberType>(
    point: &Point2<T>,
    segment: &Segment2<T>,
) -> Option<Point2<T>> {
    let v = segment.target() - segment.source();
    let w = *point - segment.source();
    let c1 = w.dot(&v);
    let c2 = v.dot(&v);
    let eps = T::default_eps();
    if c1 < -eps || c1 > c2 + eps {
        return None;
    }
    let b = c1 / c2;
    let t = v * b;
    let t = Point2::new(t.x(), t.y());
    let t = segment.source() + t;
    Some(Point2::new(t.x(), t.y()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_2_project_segment_2() {
        let point_2 = Point2::new(0.0, 0.0);
        let segment_2 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 10.0));
        let result = point_2_project_segment_2(&point_2, &segment_2);
        assert_eq!(result, Some(Point2::new(0.0, 0.0)));

        let point_2 = Point2::new(1.0, 1.0);
        let segment_2 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 10.0));
        let result = point_2_project_segment_2(&point_2, &segment_2);
        assert_eq!(result, Some(Point2::new(1.0, 1.0)));

        let point_2 = Point2::new(0.0, 2.0);
        let segment_2 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 10.0));
        let result = point_2_project_segment_2(&point_2, &segment_2);
        assert_eq!(result, Some(Point2::new(1.0, 1.0)));

        let point_2 = Point2::new(0.0, -2.0);
        let segment_2 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 10.0));
        let result = point_2_project_segment_2(&point_2, &segment_2);
        assert_eq!(result, None);
    }
}
