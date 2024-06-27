use crate::{
    algorithm::projection::point_2_line_segment_2::point_2_project_line_segment_2 as KernelPoint2ProjectLineSegment2,
    wasm::kernel::{line_segment_2::LineSegment2, point_2::Point2},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn point_2_project_line_segment_2(point: &Point2, segment: &LineSegment2) -> Option<Point2> {
    let kernel_result =
        KernelPoint2ProjectLineSegment2(&point.kernel_point_2, &segment.kernel_line_segment_2);
    match kernel_result {
        Some(kernel_point_2) => Some(Point2::new(kernel_point_2.x(), kernel_point_2.y())),
        None => None,
    }
}
