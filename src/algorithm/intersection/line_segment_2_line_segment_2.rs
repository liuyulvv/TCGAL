use crate::{
    algorithm::location::point_2_line_segment_2::is_point_2_on_line_segment_2,
    kernel::{number_type::NumberType, point_2::Point2, segment_2::Segment2},
};

pub fn is_line_segment_2_line_segment_2_intersected<T: NumberType>(
    line_segment_a: &impl Segment2<T>,
    line_segment_b: &impl Segment2<T>,
) -> bool {
    let ab = line_segment_a.target() - line_segment_a.source();
    let ac = line_segment_b.source() - line_segment_a.source();
    let ad = line_segment_b.target() - line_segment_a.source();
    let cd = line_segment_b.target() - line_segment_b.source();
    let ca = line_segment_a.source() - line_segment_b.source();
    let cb = line_segment_a.target() - line_segment_b.source();
    let eps = T::default_eps();
    ab.cross(&ac) * ab.cross(&ad) < eps && cd.cross(&ca) * cd.cross(&cb) < eps
}

pub fn line_segment_2_line_segment_2_intersection<T: NumberType>(
    line_segment_a: &impl Segment2<T>,
    line_segment_b: &impl Segment2<T>,
) -> Vec<Point2<T>> {
    let mut result = Vec::new();
    if is_line_segment_2_line_segment_2_intersected(line_segment_a, line_segment_b) {
        if is_point_2_on_line_segment_2(&line_segment_a.source(), line_segment_b) {
            result.push(line_segment_a.source());
        }
        if is_point_2_on_line_segment_2(&line_segment_a.target(), line_segment_b) {
            result.push(line_segment_a.target());
        }
        if is_point_2_on_line_segment_2(&line_segment_b.source(), line_segment_a) {
            result.push(line_segment_b.source());
        }
        if is_point_2_on_line_segment_2(&line_segment_b.target(), line_segment_a) {
            result.push(line_segment_b.target());
        }
        if result.is_empty() {
            let ac = line_segment_b.source() - line_segment_a.source();
            let ab = line_segment_a.target() - line_segment_a.source();
            let cd = line_segment_b.target() - line_segment_b.source();
            let t = ac.cross(&cd) / ab.cross(&cd);
            let res = line_segment_a.source().get_vector() + ab * t;
            let point = Point2::new(res.x(), res.y());
            result.push(point);
            return result;
        } else {
            result.sort_by(|a, b| a.partial_cmp(b).unwrap());
            result.dedup();
            return result;
        }
    }
    return result;
}

#[cfg(test)]
mod tests {

    use crate::kernel::line_segment_2::LineSegment2;

    use super::*;

    #[test]
    fn test_is_segment_2_segment_2_intersected() {
        // Segment endpoints are the same
        let line_segment_a = LineSegment2::new(Point2::new(1.0, 1.0), Point2::new(1.0, 1.0));
        let line_segment_b = LineSegment2::new(Point2::new(1.0, 1.0), Point2::new(1.0, 1.0));
        assert_eq!(
            is_line_segment_2_line_segment_2_intersected(&line_segment_a, &line_segment_b),
            true
        );

        // Segment overlap
        let line_segment_a = LineSegment2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0));
        let line_segment_b = LineSegment2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0));
        assert_eq!(
            is_line_segment_2_line_segment_2_intersected(&line_segment_a, &line_segment_b),
            true
        );

        // Segment overlap
        let line_segment_a = LineSegment2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0));
        let line_segment_b = LineSegment2::new(Point2::new(-1.0, -1.0), Point2::new(1.0, 1.0));
        assert_eq!(
            is_line_segment_2_line_segment_2_intersected(&line_segment_a, &line_segment_b),
            true
        );

        let line_segment_a = LineSegment2::new(Point2::new(10.0, 0.0), Point2::new(10.0, 10.0));
        let line_segment_b = LineSegment2::new(Point2::new(5.0, 5.0), Point2::new(10.0, 5.0));
        assert_eq!(
            is_line_segment_2_line_segment_2_intersected(&line_segment_a, &line_segment_b),
            true
        );

        let line_segment_a = LineSegment2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 10.0));
        let line_segment_b = LineSegment2::new(Point2::new(5.0, 5.0), Point2::new(5.0, 10.0));
        assert_eq!(
            is_line_segment_2_line_segment_2_intersected(&line_segment_a, &line_segment_b),
            true
        );
    }

    #[test]
    fn intersection() {
        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.0);
        let line_segment_a = LineSegment2::new(p1, p2);
        let line_segment_b = LineSegment2::new(p1, p2);
        let result = line_segment_2_line_segment_2_intersection(&line_segment_a, &line_segment_b);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Point2::new(1.0, 1.0));

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 2.0);
        let line_segment_a = LineSegment2::new(p1, p2);
        let line_segment_b = LineSegment2::new(p1, p2);
        let result = line_segment_2_line_segment_2_intersection(&line_segment_a, &line_segment_b);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Point2::new(1.0, 2.0));
        assert_eq!(result[1], Point2::new(1.0, 1.0));

        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let p3 = Point2::new(10.0, 15.0);
        let line_segment_a = LineSegment2::new(p1, p2);
        let line_segment_b = LineSegment2::new(p2, p3);
        let result = line_segment_2_line_segment_2_intersection(&line_segment_a, &line_segment_b);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Point2::new(10.0, 10.0));

        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let p3 = Point2::new(0.0, 10.0);
        let p4 = Point2::new(10.0, 0.0);
        let line_segment_a = LineSegment2::new(p1, p2);
        let line_segment_b = LineSegment2::new(p3, p4);
        let result = line_segment_2_line_segment_2_intersection(&line_segment_a, &line_segment_b);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Point2::new(5.0, 5.0));

        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let p3 = Point2::new(0.0, 5.0);
        let p4 = Point2::new(10.0, 0.0);
        let line_segment_a = LineSegment2::new(p1, p2);
        let line_segment_b = LineSegment2::new(p3, p4);
        let result = line_segment_2_line_segment_2_intersection(&line_segment_a, &line_segment_b);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Point2::new(3.333333333333333, 3.333333333333333));
    }
}
