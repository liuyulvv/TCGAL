use crate::{
    kernel::{
        line_segment_2::LineSegment2 as KernelLineSegment2, segment_2::Segment2,
        util_enum::Segment2Type,
    },
    wasm::kernel::point_2::Point2,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct LineSegment2 {
    #[wasm_bindgen(skip)]
    pub kernel_line_segment_2: KernelLineSegment2<f64>,
}

#[wasm_bindgen]
impl LineSegment2 {
    #[wasm_bindgen(constructor)]
    pub fn new(source: Point2, target: Point2) -> Self {
        Self {
            kernel_line_segment_2: KernelLineSegment2::new(
                source.kernel_point_2,
                target.kernel_point_2,
            ),
        }
    }

    pub fn segment_type(&self) -> Segment2Type {
        self.kernel_line_segment_2.segment_type()
    }

    pub fn source(&self) -> Point2 {
        Point2 {
            kernel_point_2: self.kernel_line_segment_2.source(),
        }
    }

    pub fn target(&self) -> Point2 {
        Point2 {
            kernel_point_2: self.kernel_line_segment_2.target(),
        }
    }
}
