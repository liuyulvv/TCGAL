use crate::{
    geometry::{point_2::Point2, segment_2::Segment2},
    traits::eps::Eps,
};

pub fn is_segment_2_segment_2_intersected(
    segment_2_a: &Segment2,
    segment_2_b: &Segment2,
    eps: Option<Eps>,
) -> bool {
    let eps = eps.unwrap_or(Eps::default()).value;
    let ab = segment_2_a.end - segment_2_a.start;
    let ac = segment_2_b.start - segment_2_a.start;
    let ad: crate::math::vector_2::Vector2 = segment_2_b.end - segment_2_a.start;
    let cd = segment_2_b.end - segment_2_b.start;
    let ca = segment_2_a.start - segment_2_b.start;
    let cb = segment_2_a.end - segment_2_b.start;
    ab.cross(&ac) * ab.cross(&ad) < eps && cd.cross(&ca) * cd.cross(&cb) < eps
}

/// TODO: implement overlap
/// overlay is not implemented
pub fn segment_2_segment_2_intersection_point_2(
    segment_2_a: &Segment2,
    segment_2_b: &Segment2,
    eps: Option<Eps>,
) -> Option<Point2> {
    if !is_segment_2_segment_2_intersected(segment_2_a, segment_2_b, eps) {
        return None;
    }
    let eps = eps.unwrap_or(Eps::default()).value;
    let ab = segment_2_a.end - segment_2_a.start;
    let cd = segment_2_b.end - segment_2_b.start;
    let ac = segment_2_b.start - segment_2_a.start;
    let cross_ab_cd = ab.cross(&cd);
    if cross_ab_cd.abs() < eps || cross_ab_cd.is_nan() {
        return None;
    }
    let t = ac.cross(&cd) / ab.cross(&cd);
    let res = segment_2_a.start.get_vector() + ab * t;
    return Some(Point2::from_vector(&res));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_intersect() {
        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p1, p2);
        let result = is_segment_2_segment_2_intersected(&s1, &s2, None);
        assert_eq!(result, true);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p1, p2);
        let result = is_segment_2_segment_2_intersected(&s1, &s2, None);
        assert_eq!(result, true);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p2, p1);
        let result = is_segment_2_segment_2_intersected(&s1, &s2, None);
        assert_eq!(result, true);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let p3 = Point2::new(1.1, 1.2);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p2, p3);
        let result = is_segment_2_segment_2_intersected(&s1, &s2, None);
        assert_eq!(result, true);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let p3 = Point2::new(1.1, 1.2);
        let p4 = Point2::new(1.2, 1.3);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p3, p4);
        let result = is_segment_2_segment_2_intersected(&s1, &s2, None);
        assert_eq!(result, false);
    }

    #[test]
    fn intersection() {
        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p1, p2);
        let result = segment_2_segment_2_intersection_point_2(&s1, &s2, None);
        assert_eq!(result, None);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 2.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p1, p2);
        let result = segment_2_segment_2_intersection_point_2(&s1, &s2, None);
        assert_eq!(result, None);

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let p3 = Point2::new(1.1, 1.2);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p2, p3);
        let result = segment_2_segment_2_intersection_point_2(&s1, &s2, None);
        assert_eq!(result, Some(Point2::new(1.0, 1.1)));

        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.1);
        let p3 = Point2::new(1.1, 1.2);
        let p4 = Point2::new(1.2, 1.3);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p3, p4);
        let result = segment_2_segment_2_intersection_point_2(&s1, &s2, None);
        assert_eq!(result, None);

        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let p3 = Point2::new(0.0, 5.0);
        let p4 = Point2::new(10.0, 5.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p3, p4);
        let result = segment_2_segment_2_intersection_point_2(&s1, &s2, None);
        assert_eq!(result, Some(Point2::new(5.0, 5.0)));

        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let p3 = Point2::new(0.0, 5.0);
        let p4 = Point2::new(10.0, 0.0);
        let s1 = Segment2::new(p1, p2);
        let s2 = Segment2::new(p3, p4);
        let result = segment_2_segment_2_intersection_point_2(&s1, &s2, None);
        assert_eq!(result, Some(Point2::new(3.333333, 3.333333)));
    }
}
