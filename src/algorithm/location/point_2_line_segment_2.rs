use crate::kernel::{number_type::NumberType, point_2::Point2, segment_2::Segment2};

use super::location_enum::Point2Segment2Location;

pub fn is_point_2_on_line_segment_2<T: NumberType>(
    point_2: &Point2<T>,
    segment_2: &impl Segment2<T>,
) -> bool {
    let location = locate_point_2_line_segment_2(point_2, segment_2);
    match location {
        Point2Segment2Location::On => true,
        _ => false,
    }
}

pub fn locate_point_2_line_segment_2<T: NumberType>(
    point_2: &Point2<T>,
    segment_2: &impl Segment2<T>,
) -> Point2Segment2Location {
    let source = segment_2.source();
    let target = segment_2.target();
    let vec_ab = target - source;
    let vec_ac = *point_2 - source;
    let area = vec_ab.cross(&vec_ac);
    if area > T::default_eps() {
        Point2Segment2Location::Left
    } else if area < -T::default_eps() {
        Point2Segment2Location::Right
    } else {
        let dot_ab_ac = vec_ab.dot(&vec_ac);
        let dot_ab_ab = vec_ab.dot(&vec_ab);
        if dot_ab_ac <= dot_ab_ab && dot_ab_ac >= -T::default_eps() {
            Point2Segment2Location::On
        } else {
            Point2Segment2Location::Collinear
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::kernel::line_segment_2::LineSegment2;

    use super::*;

    #[test]
    fn test_is_point_2_on_line_segment_2() {
        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let s = LineSegment2::new(p1, p2);

        assert_eq!(is_point_2_on_line_segment_2(&p1, &s), true);

        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let s = LineSegment2::new(p1, p2);

        let p3 = Point2::new(5.0, 5.0);
        assert_eq!(is_point_2_on_line_segment_2(&p3, &s), true);

        let p3 = Point2::new(5.0, 5.1);
        assert_eq!(is_point_2_on_line_segment_2(&p3, &s), false);

        let p3 = Point2::new(f64::sqrt(3.0), f64::sqrt(3.0));
        assert_eq!(is_point_2_on_line_segment_2(&p3, &s), true);
    }

    #[test]
    fn test_locate_point_2_line_segment_2() {
        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let s = LineSegment2::new(p1, p2);

        let p3 = Point2::new(5.0, 5.0);
        assert_eq!(
            locate_point_2_line_segment_2(&p3, &s),
            Point2Segment2Location::On
        );

        let p3 = Point2::new(5.0, 5.1);
        assert_eq!(
            locate_point_2_line_segment_2(&p3, &s),
            Point2Segment2Location::Left
        );

        let p3 = Point2::new(5.0, 4.9);
        assert_eq!(
            locate_point_2_line_segment_2(&p3, &s),
            Point2Segment2Location::Right
        );

        let p3 = Point2::new(0.0, 0.0);
        assert_eq!(
            locate_point_2_line_segment_2(&p3, &s),
            Point2Segment2Location::On
        );

        let p3 = Point2::new(10.0, 10.0);
        assert_eq!(
            locate_point_2_line_segment_2(&p3, &s),
            Point2Segment2Location::On
        );

        let p3 = Point2::new(10.0, 10.1);
        assert_eq!(
            locate_point_2_line_segment_2(&p3, &s),
            Point2Segment2Location::Left
        );

        let p3 = Point2::new(10.0, 9.9);
        assert_eq!(
            locate_point_2_line_segment_2(&p3, &s),
            Point2Segment2Location::Right
        );

        let p3 = Point2::new(-10.0, -10.0);
        assert_eq!(
            locate_point_2_line_segment_2(&p3, &s),
            Point2Segment2Location::Collinear
        );
    }
}
