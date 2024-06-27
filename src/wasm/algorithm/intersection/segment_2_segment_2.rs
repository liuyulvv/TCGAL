use crate::{
    algorithm::intersection::segment_2_segment_2::{
        is_segment_2_segment_2_intersected as KernelIsSegment2Segment2Intersected,
        segment_2_segment_2_intersection as KernelSegment2Segment2Intersection,
    },
    wasm::kernel::{
        point_2::Point2,
        segment_2::{get_segment_value, Segment2, Segment2Value},
    },
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_segment_2_segment_2_intersected(segment_a: &Segment2, segment_b: &Segment2) -> bool {
    let segment_a = get_segment_value(segment_a);
    let segment_b = get_segment_value(segment_b);
    match segment_a {
        Segment2Value::LineSegment2(segment_a) => match segment_b {
            Segment2Value::LineSegment2(segment2_b) => KernelIsSegment2Segment2Intersected(
                &segment_a.kernel_line_segment_2,
                &segment2_b.kernel_line_segment_2,
            ),
            Segment2Value::CircleSegment2(segment2_b) => KernelIsSegment2Segment2Intersected(
                &segment_a.kernel_line_segment_2,
                &segment2_b.kernel_circle_segment_2,
            ),
            Segment2Value::ArcSegment2(segment2_b) => KernelIsSegment2Segment2Intersected(
                &segment_a.kernel_line_segment_2,
                &segment2_b.kernel_arc_segment_2,
            ),
        },
        Segment2Value::CircleSegment2(segment_a) => match segment_b {
            Segment2Value::LineSegment2(segment2_b) => KernelIsSegment2Segment2Intersected(
                &segment_a.kernel_circle_segment_2,
                &segment2_b.kernel_line_segment_2,
            ),
            Segment2Value::CircleSegment2(segment2_b) => KernelIsSegment2Segment2Intersected(
                &segment_a.kernel_circle_segment_2,
                &segment2_b.kernel_circle_segment_2,
            ),
            Segment2Value::ArcSegment2(segment2_b) => KernelIsSegment2Segment2Intersected(
                &segment_a.kernel_circle_segment_2,
                &segment2_b.kernel_arc_segment_2,
            ),
        },
        Segment2Value::ArcSegment2(segment_a) => match segment_b {
            Segment2Value::LineSegment2(segment2_b) => KernelIsSegment2Segment2Intersected(
                &segment_a.kernel_arc_segment_2,
                &segment2_b.kernel_line_segment_2,
            ),
            Segment2Value::CircleSegment2(segment2_b) => KernelIsSegment2Segment2Intersected(
                &segment_a.kernel_arc_segment_2,
                &segment2_b.kernel_circle_segment_2,
            ),
            Segment2Value::ArcSegment2(segment2_b) => KernelIsSegment2Segment2Intersected(
                &segment_a.kernel_arc_segment_2,
                &segment2_b.kernel_arc_segment_2,
            ),
        },
    }
}

#[wasm_bindgen]
pub fn segment_2_segment_2_intersection(segment_a: &Segment2, segment_b: &Segment2) -> Vec<Point2> {
    let segment_a = get_segment_value(segment_a);
    let segment_b = get_segment_value(segment_b);
    let result = match segment_a {
        Segment2Value::LineSegment2(segment_a) => match segment_b {
            Segment2Value::LineSegment2(segment2_b) => KernelSegment2Segment2Intersection(
                &segment_a.kernel_line_segment_2,
                &segment2_b.kernel_line_segment_2,
            ),
            Segment2Value::CircleSegment2(segment2_b) => KernelSegment2Segment2Intersection(
                &segment_a.kernel_line_segment_2,
                &segment2_b.kernel_circle_segment_2,
            ),
            Segment2Value::ArcSegment2(segment2_b) => KernelSegment2Segment2Intersection(
                &segment_a.kernel_line_segment_2,
                &segment2_b.kernel_arc_segment_2,
            ),
        },
        Segment2Value::CircleSegment2(segment_a) => match segment_b {
            Segment2Value::LineSegment2(segment2_b) => KernelSegment2Segment2Intersection(
                &segment_a.kernel_circle_segment_2,
                &segment2_b.kernel_line_segment_2,
            ),
            Segment2Value::CircleSegment2(segment2_b) => KernelSegment2Segment2Intersection(
                &segment_a.kernel_circle_segment_2,
                &segment2_b.kernel_circle_segment_2,
            ),
            Segment2Value::ArcSegment2(segment2_b) => KernelSegment2Segment2Intersection(
                &segment_a.kernel_circle_segment_2,
                &segment2_b.kernel_arc_segment_2,
            ),
        },
        Segment2Value::ArcSegment2(segment_a) => match segment_b {
            Segment2Value::LineSegment2(segment2_b) => KernelSegment2Segment2Intersection(
                &segment_a.kernel_arc_segment_2,
                &segment2_b.kernel_line_segment_2,
            ),
            Segment2Value::CircleSegment2(segment2_b) => KernelSegment2Segment2Intersection(
                &segment_a.kernel_arc_segment_2,
                &segment2_b.kernel_circle_segment_2,
            ),
            Segment2Value::ArcSegment2(segment2_b) => KernelSegment2Segment2Intersection(
                &segment_a.kernel_arc_segment_2,
                &segment2_b.kernel_arc_segment_2,
            ),
        },
    };
    result
        .into_iter()
        .map(|p| Point2 { kernel_point_2: p })
        .collect()
}
