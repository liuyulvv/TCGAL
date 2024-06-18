use crate::kernel::{
    number_type::NumberType, point_2::Point2, segment_2::Segment2, util_enum::Segment2Type,
};

use super::{
    arc_segment_2_arc_segment_2::is_arc_segment_2_arc_segment_2_intersected,
    circle_segment_2_circle_segment_2::{
        circle_segment_2_circle_segment_2_intersection,
        is_circle_segment_2_circle_segment_2_intersected,
    },
    line_segment_2_line_segment_2::{
        is_line_segment_2_line_segment_2_intersected, line_segment_2_line_segment_2_intersection,
    },
};

pub fn is_segment_2_segment_2_intersected<T: NumberType>(
    segment_a: &impl Segment2<T>,
    segment_b: &impl Segment2<T>,
) -> bool {
    let segment_a_type = segment_a.segment_type();
    let segment_b_type = segment_b.segment_type();
    if segment_a_type == segment_b_type {
        match segment_a_type {
            Segment2Type::LineSegment2 => {
                is_line_segment_2_line_segment_2_intersected(segment_a, segment_b)
            }
            Segment2Type::CircleSegment2 => {
                is_circle_segment_2_circle_segment_2_intersected(segment_a, segment_b)
            }
            Segment2Type::ArcSegment2 => {
                is_arc_segment_2_arc_segment_2_intersected(segment_a, segment_b)
            }
        }
    } else {
        todo!()
    }
}

pub fn segment_2_segment_2_intersection<T: NumberType>(
    segment_a: &impl Segment2<T>,
    segment_b: &impl Segment2<T>,
) -> Vec<Point2<T>> {
    let segment_a_type = segment_a.segment_type();
    let segment_b_type = segment_b.segment_type();
    if segment_a_type == segment_b_type {
        match segment_a_type {
            Segment2Type::LineSegment2 => {
                line_segment_2_line_segment_2_intersection(segment_a, segment_b)
            }
            Segment2Type::CircleSegment2 => {
                circle_segment_2_circle_segment_2_intersection(segment_a, segment_b)
            }
            Segment2Type::ArcSegment2 => {
                todo!()
            }
        }
    } else {
        todo!()
    }
}
