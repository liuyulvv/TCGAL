use crate::{
    algorithm::{
        intersection::circle_segment_2_circle_segment_2::circle_segment_2_circle_segment_2_intersection,
        location::{
            location_enum::Point2ArcSegment2Location,
            point_2_arc_segment_2::locate_point_2_arc_segment_2,
        },
    },
    kernel::{
        circle_segment_2::CircleSegment2, number_type::NumberType, point_2::Point2,
        segment_2::Segment2,
    },
};

pub fn is_arc_segment_2_arc_segment_2_intersected<T: NumberType>(
    arc_segment_a: &impl Segment2<T>,
    arc_segment_b: &impl Segment2<T>,
) -> bool {
    let result = arc_segment_2_arc_segment_2_intersection(arc_segment_a, arc_segment_b);
    return !result.is_empty();
}

pub fn arc_segment_2_arc_segment_2_intersection<T: NumberType>(
    arc_segment_a: &impl Segment2<T>,
    arc_segment_b: &impl Segment2<T>,
) -> Vec<Point2<T>> {
    let circle_segment_a = CircleSegment2::new(arc_segment_a.center(), arc_segment_a.radius());
    let circle_segment_b = CircleSegment2::new(arc_segment_b.center(), arc_segment_b.radius());
    let points =
        circle_segment_2_circle_segment_2_intersection(&circle_segment_a, &circle_segment_b);
    let mut result = Vec::new();
    for point in &points {
        let on_arc_segment_a = locate_point_2_arc_segment_2(point, arc_segment_a);
        let on_arc_segment_b = locate_point_2_arc_segment_2(point, arc_segment_b);
        if on_arc_segment_a == on_arc_segment_b && on_arc_segment_a == Point2ArcSegment2Location::On
        {
            result.push(point.clone());
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::kernel::arc_segment_2::ArcSegment2;

    use super::*;

    #[test]
    fn test_is_arc_segment_2_arc_segment_2_intersected() {
        let arc_segment_a =
            ArcSegment2::new(CircleSegment2::new(Point2::new(0.0, 0.0), 5.0), 0.0, PI);

        let arc_segment_b =
            ArcSegment2::new(CircleSegment2::new(Point2::new(0.0, 0.0), 5.0), 0.0, PI);
        assert_eq!(
            is_arc_segment_2_arc_segment_2_intersected(&arc_segment_a, &arc_segment_b),
            false
        );

        let arc_segment_b =
            ArcSegment2::new(CircleSegment2::new(Point2::new(5.0, 0.0), 5.0), 0.0, PI);
        assert_eq!(
            is_arc_segment_2_arc_segment_2_intersected(&arc_segment_a, &arc_segment_b),
            true
        );
    }

    #[test]
    fn test_arc_segment_2_arc_segment_2_intersection() {
        let arc_segment_a =
            ArcSegment2::new(CircleSegment2::new(Point2::new(0.0, 0.0), 5.0), 0.0, PI);

        let arc_segment_b =
            ArcSegment2::new(CircleSegment2::new(Point2::new(0.0, 0.0), 5.0), 0.0, PI);
        assert_eq!(
            arc_segment_2_arc_segment_2_intersection(&arc_segment_a, &arc_segment_b),
            Vec::new()
        );

        let arc_segment_b =
            ArcSegment2::new(CircleSegment2::new(Point2::new(5.0, 0.0), 5.0), 0.0, PI);
        assert_eq!(
            arc_segment_2_arc_segment_2_intersection(&arc_segment_a, &arc_segment_b),
            vec![Point2::new(2.5, 2.5 * 3.0.sqrt()),]
        );
    }
}
