use crate::kernel::{
    circle_segment_2::CircleSegment2, number_type::NumberType, point_2::Point2, segment_2::Segment2,
};

use super::location_enum::Point2Circle2Location;

pub fn is_point_2_on_circle_segment_2<T: NumberType>(
    point_2: &Point2<T>,
    circle_2: &CircleSegment2<T>,
) -> bool {
    let location = locate_point_2_circle_segment_2(point_2, circle_2);
    match location {
        Point2Circle2Location::On => true,
        _ => false,
    }
}

pub fn locate_point_2_circle_segment_2<T: NumberType>(
    point_2: &Point2<T>,
    circle_2: &CircleSegment2<T>,
) -> Point2Circle2Location {
    let center = circle_2.center();
    let radius = circle_2.radius();
    let vec_center_point = *point_2 - center;
    let distance = vec_center_point.length();
    let eps = T::default_eps();
    if (distance - radius).abs() < eps {
        Point2Circle2Location::On
    } else if distance < radius {
        Point2Circle2Location::Inside
    } else {
        Point2Circle2Location::Outside
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_point_2_on_circle_segment_2() {
        let circle_2 = CircleSegment2::new(Point2::new(0.0, 0.0), 10.0);

        let point_2 = Point2::new(0.0, 10.0);
        assert_eq!(is_point_2_on_circle_segment_2(&point_2, &circle_2), true);

        let point_2 = Point2::new(0.0, 10.1);
        assert_eq!(is_point_2_on_circle_segment_2(&point_2, &circle_2), false);

        let point_2 = Point2::new(0.0, 9.9);
        assert_eq!(is_point_2_on_circle_segment_2(&point_2, &circle_2), false);

        let point_2 = Point2::new(0.0, 0.0);
        assert_eq!(is_point_2_on_circle_segment_2(&point_2, &circle_2), false);

        let point_2 = Point2::new(5.0 * f64::sqrt(2.0), 5.0 * f64::sqrt(2.0));
        assert_eq!(is_point_2_on_circle_segment_2(&point_2, &circle_2), true);
    }

    #[test]
    fn test_locate_point_2_circle_segment_2() {
        let circle_2 = CircleSegment2::new(Point2::new(0.0, 0.0), 10.0);

        let point_2 = Point2::new(0.0, 10.0);
        assert_eq!(
            locate_point_2_circle_segment_2(&point_2, &circle_2),
            Point2Circle2Location::On
        );

        let point_2 = Point2::new(0.0, 10.1);
        assert_eq!(
            locate_point_2_circle_segment_2(&point_2, &circle_2),
            Point2Circle2Location::Outside
        );

        let point_2 = Point2::new(0.0, 9.9);
        assert_eq!(
            locate_point_2_circle_segment_2(&point_2, &circle_2),
            Point2Circle2Location::Inside
        );

        let point_2 = Point2::new(0.0, 0.0);
        assert_eq!(
            locate_point_2_circle_segment_2(&point_2, &circle_2),
            Point2Circle2Location::Inside
        );

        let point_2 = Point2::new(5.0 * f64::sqrt(2.0), 5.0 * f64::sqrt(2.0));
        assert_eq!(
            locate_point_2_circle_segment_2(&point_2, &circle_2),
            Point2Circle2Location::On
        );
    }
}
