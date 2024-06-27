use crate::{
    kernel::{point_2::Point2 as KernelPoint2, util_enum::TurnDirection},
    wasm::kernel::vector_2::Vector2,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point2 {
    #[wasm_bindgen(skip)]
    pub kernel_point_2: KernelPoint2<f64>,
}

#[wasm_bindgen]
impl Point2 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            kernel_point_2: KernelPoint2::new(x, y),
        }
    }

    pub fn x(&self) -> f64 {
        self.kernel_point_2.x()
    }

    pub fn y(&self) -> f64 {
        self.kernel_point_2.y()
    }

    pub fn equals(&self, other: &Point2) -> bool {
        self.kernel_point_2.equals(&other.kernel_point_2)
    }

    pub fn get_vector(&self) -> Vector2 {
        Vector2::new(self.x(), self.y())
    }

    pub fn distance(&self, other: &Point2) -> f64 {
        self.kernel_point_2.distance(&other.kernel_point_2)
    }

    pub fn turn(p: &Point2, q: &Point2, r: &Point2) -> TurnDirection {
        KernelPoint2::turn(&p.kernel_point_2, &q.kernel_point_2, &r.kernel_point_2)
    }
}
