use crate::{
    kernel::{
        circle_segment_2::CircleSegment2 as KernelCircleSegment2, segment_2::Segment2,
        util_enum::Segment2Type,
    },
    wasm::kernel::point_2::Point2,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct CircleSegment2 {
    #[wasm_bindgen(skip)]
    pub kernel_circle_segment_2: KernelCircleSegment2<f64>,
}

#[wasm_bindgen]
impl CircleSegment2 {
    #[wasm_bindgen(constructor)]
    pub fn new(center: Point2, radius: f64) -> Self {
        Self {
            kernel_circle_segment_2: KernelCircleSegment2::new(center.kernel_point_2, radius),
        }
    }

    pub fn segment_type(&self) -> Segment2Type {
        self.kernel_circle_segment_2.segment_type()
    }

    pub fn center(&self) -> Point2 {
        Point2 {
            kernel_point_2: self.kernel_circle_segment_2.center(),
        }
    }

    pub fn radius(&self) -> f64 {
        self.kernel_circle_segment_2.radius()
    }
}
