use crate::{
    algorithm::location::{
        location_enum::{Point2ArcSegment2Location, Point2Circle2Location},
        point_2_arc_segment_2::locate_point_2_arc_segment_2,
        point_2_circle_segment_2::locate_point_2_circle_segment_2,
    },
    kernel::{
        circle_segment_2::CircleSegment2, number_type::NumberType, point_2::Point2,
        segment_2::Segment2,
    },
};

use super::circle_segment_2_circle_segment_2::circle_segment_2_circle_segment_2_intersection;

pub fn is_circle_segment_2_arc_segment_2_intersected<T: NumberType>(
    circle_segment: &impl Segment2<T>,
    arc_segment: &impl Segment2<T>,
) -> bool {
    let points = circle_segment_2_arc_segment_2_intersection(circle_segment, arc_segment);
    !points.is_empty()
}

pub fn circle_segment_2_arc_segment_2_intersection<T: NumberType>(
    circle_segment: &impl Segment2<T>,
    arc_segment: &impl Segment2<T>,
) -> Vec<Point2<T>> {
    let support_circle_segment = CircleSegment2::new(arc_segment.center(), arc_segment.radius());
    let points =
        circle_segment_2_circle_segment_2_intersection(&support_circle_segment, circle_segment);
    let mut result = Vec::new();
    for point in &points {
        let location_arc = locate_point_2_arc_segment_2(point, arc_segment);
        let location_circle = locate_point_2_circle_segment_2(point, circle_segment);
        if location_arc == Point2ArcSegment2Location::On
            && location_circle == Point2Circle2Location::On
        {
            result.push(point.clone());
        }
    }
    result
}
