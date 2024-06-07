use crate::{
    algorithm::location::point_2_segment_2::is_point_2_on_segment_2,
    kernel::{number_type::NumberType, point_2::Point2, segment_2::Segment2},
};

pub fn is_segment_2_segment_2_intersected<T: NumberType>(
    s1: &Segment2<T>,
    s2: &Segment2<T>,
) -> bool {
    let ab = s1.target() - s1.source();
    let ac = s2.source() - s1.source();
    let ad = s2.target() - s1.source();
    let cd = s2.target() - s2.source();
    let ca = s1.source() - s2.source();
    let cb = s1.target() - s2.source();
    let eps = T::default_eps();
    ab.cross(&ac) * ab.cross(&ad) < eps && cd.cross(&ca) * cd.cross(&cb) < eps
}

pub fn segment_2_segment_2_intersection<T: NumberType>(
    s1: &Segment2<T>,
    s2: &Segment2<T>,
) -> Vec<Point2<T>> {
    let mut result = Vec::new();
    if is_segment_2_segment_2_intersected(s1, s2) {
        if is_point_2_on_segment_2(&s1.source(), s2) {
            result.push(s1.source());
        }
        if is_point_2_on_segment_2(&s1.target(), s2) {
            result.push(s1.target());
        }
        if is_point_2_on_segment_2(&s2.source(), s1) {
            result.push(s2.source());
        }
        if is_point_2_on_segment_2(&s2.target(), s1) {
            result.push(s2.target());
        }
        if result.is_empty() {
            let ac = s2.source() - s1.source();
            let ab = s1.target() - s1.source();
            let cd = s2.target() - s2.source();
            let t = ac.cross(&cd) / ab.cross(&cd);
            let res = s1.source().get_vector() + ab * t;
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

    use super::*;

    #[test]
    fn test_is_segment_2_segment_2_intersected() {
        // Segment endpoints are the same
        let s1 = Segment2::new(Point2::new(1.0, 1.0), Point2::new(1.0, 1.0));
        let s2 = Segment2::new(Point2::new(1.0, 1.0), Point2::new(1.0, 1.0));
        assert_eq!(is_segment_2_segment_2_intersected(&s1, &s2), true);

        // Segment overlap
        let s1 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0));
        let s2 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0));
        assert_eq!(is_segment_2_segment_2_intersected(&s1, &s2), true);

        // Segment overlap
        let s1 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0));
        let s2 = Segment2::new(Point2::new(-1.0, -1.0), Point2::new(1.0, 1.0));
        assert_eq!(is_segment_2_segment_2_intersected(&s1, &s2), true);

        let s1 = Segment2::new(Point2::new(10.0, 0.0), Point2::new(10.0, 10.0));
        let s2 = Segment2::new(Point2::new(5.0, 5.0), Point2::new(10.0, 5.0));
        assert_eq!(is_segment_2_segment_2_intersected(&s1, &s2), true);

        let s1 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 10.0));
        let s2 = Segment2::new(Point2::new(5.0, 5.0), Point2::new(5.0, 10.0));
        assert_eq!(is_segment_2_segment_2_intersected(&s1, &s2), true);
    }

    #[test]
    fn intersection() {
        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p1, p2);
        let result = segment_2_segment_2_intersection(&s1, &s2);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Point2::new(1.0, 1.0));

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 2.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p1, p2);
        let result = segment_2_segment_2_intersection(&s1, &s2);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Point2::new(1.0, 1.0));
        assert_eq!(result[1], Point2::new(1.0, 2.0));

        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let p3 = Point2::new(10.0, 15.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p2, p3);
        let result = segment_2_segment_2_intersection(&s1, &s2);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Point2::new(10.0, 10.0));

        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let p3 = Point2::new(0.0, 10.0);
        let p4 = Point2::new(10.0, 0.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p3, p4);
        let result = segment_2_segment_2_intersection(&s1, &s2);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Point2::new(5.0, 5.0));

        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let p3 = Point2::new(0.0, 5.0);
        let p4 = Point2::new(10.0, 0.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p3, p4);
        let result = segment_2_segment_2_intersection(&s1, &s2);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Point2::new(3.333333333333333, 3.333333333333333));
    }
}
