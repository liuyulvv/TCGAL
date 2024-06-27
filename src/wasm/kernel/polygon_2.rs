use crate::{
    kernel::polygon_2::Polygon2 as KernelPolygon2,
    wasm::kernel::{line_segment_2::LineSegment2, point_2::Point2},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Polygon2 {
    #[wasm_bindgen(skip)]
    pub kernel_polygon_2: KernelPolygon2<f64>,
}

#[wasm_bindgen]
impl Polygon2 {
    #[wasm_bindgen(constructor)]
    pub fn new(points: Vec<Point2>) -> Self {
        Self {
            kernel_polygon_2: KernelPolygon2::new(
                points.iter().map(|point| point.kernel_point_2).collect(),
            ),
        }
    }

    pub fn vertices(&self) -> Vec<Point2> {
        self.kernel_polygon_2
            .vertices()
            .iter()
            .map(|point| Point2 {
                kernel_point_2: point.clone(),
            })
            .collect()
    }

    pub fn edges(&self) -> Vec<LineSegment2> {
        self.kernel_polygon_2
            .edges()
            .iter()
            .map(|segment| LineSegment2 {
                kernel_line_segment_2: segment.clone(),
            })
            .collect()
    }

    pub fn area(&self) -> f64 {
        self.kernel_polygon_2.area()
    }

    pub fn is_simple(&self) -> bool {
        self.kernel_polygon_2.is_simple()
    }

    pub fn is_convex(&self) -> bool {
        self.kernel_polygon_2.is_convex()
    }
}
