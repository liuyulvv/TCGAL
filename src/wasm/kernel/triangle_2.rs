use crate::{
    kernel::{triangle_2::Triangle2 as KernelTriangle2, util_enum::Orientation},
    wasm::kernel::{line_segment_2::LineSegment2, point_2::Point2},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Triangle2 {
    #[wasm_bindgen(skip)]
    pub kernel_triangle_2: KernelTriangle2<f64>,
}

#[wasm_bindgen]
impl Triangle2 {
    #[wasm_bindgen(constructor)]
    pub fn new(a: Point2, b: Point2, c: Point2) -> Triangle2 {
        Triangle2 {
            kernel_triangle_2: KernelTriangle2::new(
                a.kernel_point_2,
                b.kernel_point_2,
                c.kernel_point_2,
            ),
        }
    }

    pub fn a(&self) -> Point2 {
        Point2 {
            kernel_point_2: self.kernel_triangle_2.a(),
        }
    }

    pub fn b(&self) -> Point2 {
        Point2 {
            kernel_point_2: self.kernel_triangle_2.b(),
        }
    }

    pub fn c(&self) -> Point2 {
        Point2 {
            kernel_point_2: self.kernel_triangle_2.c(),
        }
    }

    pub fn vertices(&self) -> Vec<Point2> {
        self.kernel_triangle_2
            .vertices()
            .iter()
            .map(|point| Point2 {
                kernel_point_2: point.clone(),
            })
            .collect()
    }

    pub fn edges(&self) -> Vec<LineSegment2> {
        self.kernel_triangle_2
            .edges()
            .iter()
            .map(|segment| LineSegment2 {
                kernel_line_segment_2: segment.clone(),
            })
            .collect()
    }

    pub fn orientation(&self) -> Orientation {
        self.kernel_triangle_2.orientation()
    }

    pub fn reverse_orientation(&mut self) {
        self.kernel_triangle_2.reverse_orientation();
    }

    pub fn area(&self) -> f64 {
        self.kernel_triangle_2.area()
    }
}
