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
