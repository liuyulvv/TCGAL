use crate::{
    algorithm::location::{
        location_enum::Point2Circle2Location,
        point_2_circle_segment_2::{
            is_point_2_on_circle_segment_2 as KernelIsPoint2OnCircleSegment2,
            locate_point_2_circle_segment_2 as KernelLocatePoint2CircleSegment2,
        },
    },
    wasm::kernel::{circle_segment_2::CircleSegment2, point_2::Point2},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_point_2_on_circle_segment_2(point: &Point2, segment: &CircleSegment2) -> bool {
    KernelIsPoint2OnCircleSegment2(&point.kernel_point_2, &segment.kernel_circle_segment_2)
}

#[wasm_bindgen]
pub fn locate_point_2_circle_segment_2(
    point: &Point2,
    segment: &CircleSegment2,
) -> Point2Circle2Location {
    KernelLocatePoint2CircleSegment2(&point.kernel_point_2, &segment.kernel_circle_segment_2)
}
