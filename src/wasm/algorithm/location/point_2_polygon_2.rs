use crate::{
    algorithm::location::{
        location_enum::Point2Polygon2Location,
        point_2_polygon_2::locate_point_2_polygon_2 as KernelLocatePoint2Polygon2,
    },
    wasm::kernel::{point_2::Point2, polygon_2::Polygon2},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn locate_point_2_polygon_2(point: &Point2, polygon: &Polygon2) -> Point2Polygon2Location {
    KernelLocatePoint2Polygon2(&point.kernel_point_2, &polygon.kernel_polygon_2)
}
