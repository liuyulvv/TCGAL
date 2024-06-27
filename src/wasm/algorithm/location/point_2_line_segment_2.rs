use crate::{
    algorithm::location::{
        location_enum::Point2Segment2Location,
        point_2_line_segment_2::{
            is_point_2_on_line_segment_2 as KernelIsPoint2OnLineSegment2,
            locate_point_2_line_segment_2 as KernelLocatePoint2LineSegment2,
        },
    },
    wasm::kernel::{line_segment_2::LineSegment2, point_2::Point2},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_point_2_on_line_segment_2(point: &Point2, segment: &LineSegment2) -> bool {
    KernelIsPoint2OnLineSegment2(&point.kernel_point_2, &segment.kernel_line_segment_2)
}

#[wasm_bindgen]
pub fn locate_point_2_line_segment_2(
    point: &Point2,
    segment: &LineSegment2,
) -> Point2Segment2Location {
    KernelLocatePoint2LineSegment2(&point.kernel_point_2, &segment.kernel_line_segment_2)
}
