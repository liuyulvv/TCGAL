use crate::{
    algorithm::convex_hull::convex_hull_2::ConvexHull2 as KernelConvexHull2,
    wasm::kernel::point_2::Point2,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

pub struct ConvexHull2 {
    #[wasm_bindgen(skip)]
    pub kernel_convex_hull_2: KernelConvexHull2<f64>,
}

#[wasm_bindgen]
impl ConvexHull2 {
    #[wasm_bindgen(constructor)]
    pub fn new(points: Vec<Point2>) -> Self {
        Self {
            kernel_convex_hull_2: KernelConvexHull2::new(
                points
                    .into_iter()
                    .map(|point| point.kernel_point_2)
                    .collect(),
            ),
        }
    }

    pub fn convex_hull(&mut self) -> Vec<Point2> {
        self.kernel_convex_hull_2
            .convex_hull()
            .into_iter()
            .map(|point| Point2 {
                kernel_point_2: point,
            })
            .collect()
    }
}
