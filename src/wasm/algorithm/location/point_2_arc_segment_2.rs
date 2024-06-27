use crate::{
    algorithm::location::{
        location_enum::Point2ArcSegment2Location,
        point_2_arc_segment_2::{
            is_point_2_on_arc_segment_2 as KernelIsPoint2OnArcSegment2,
            locate_point_2_arc_segment_2 as KernelLocatePoint2ArcSegment2,
        },
    },
    wasm::kernel::{arc_segment_2::ArcSegment2, point_2::Point2},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_point_2_on_arc_segment_2(point: &Point2, segment: &ArcSegment2) -> bool {
    KernelIsPoint2OnArcSegment2(&point.kernel_point_2, &segment.kernel_arc_segment_2)
}

#[wasm_bindgen]
pub fn locate_point_2_arc_segment_2(
    point: &Point2,
    segment: &ArcSegment2,
) -> Point2ArcSegment2Location {
    KernelLocatePoint2ArcSegment2(&point.kernel_point_2, &segment.kernel_arc_segment_2)
}
