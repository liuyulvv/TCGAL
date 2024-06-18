use crate::{
    algorithm::{
        intersection::line_segment_2_circle_segment_2::line_segment_2_circle_segment_2_intersection,
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

pub fn is_line_segment_2_arc_segment_2_intersected<T: NumberType>(
    lien_segment: &impl Segment2<T>,
    arc_segment: &impl Segment2<T>,
) -> bool {
    let points = line_segment_2_arc_segment_2_intersection(lien_segment, arc_segment);
    !points.is_empty()
}

pub fn line_segment_2_arc_segment_2_intersection<T: NumberType>(
    line_segment: &impl Segment2<T>,
    arc_segment: &impl Segment2<T>,
) -> Vec<Point2<T>> {
    let circle_segment = CircleSegment2::new(arc_segment.center(), arc_segment.radius());
    let points = line_segment_2_circle_segment_2_intersection(line_segment, &circle_segment);
    let mut result = Vec::new();
    for point in &points {
        let location = locate_point_2_arc_segment_2(point, arc_segment);
        if location == Point2ArcSegment2Location::On {
            result.push(point.clone());
        }
    }
    result
}
