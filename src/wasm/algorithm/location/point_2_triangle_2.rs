use crate::{
    algorithm::location::{
        location_enum::Point2Triangle2Location,
        point_2_triangle_2::locate_point_2_triangle_2 as KernelLocatePoint2Triangle2,
    },
    wasm::kernel::{point_2::Point2, triangle_2::Triangle2},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn locate_point_2_triangle_2(point: &Point2, triangle: &Triangle2) -> Point2Triangle2Location {
    KernelLocatePoint2Triangle2(&point.kernel_point_2, &triangle.kernel_triangle_2)
}
