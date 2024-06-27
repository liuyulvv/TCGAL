use crate::{
    kernel::{
        arc_segment_2::ArcSegment2 as KernelArcSegment2, segment_2::Segment2,
        util_enum::Segment2Type,
    },
    wasm::kernel::{circle_segment_2::CircleSegment2, point_2::Point2},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct ArcSegment2 {
    #[wasm_bindgen(skip)]
    pub kernel_arc_segment_2: KernelArcSegment2<f64>,
}

#[wasm_bindgen]
impl ArcSegment2 {
    #[wasm_bindgen(constructor)]
    pub fn new(support: CircleSegment2, source_radian: f64, target_radian: f64) -> Self {
        Self {
            kernel_arc_segment_2: KernelArcSegment2::new(
                support.kernel_circle_segment_2,
                source_radian,
                target_radian,
            ),
        }
    }

    pub fn segment_type(&self) -> Segment2Type {
        self.kernel_arc_segment_2.segment_type()
    }

    pub fn source(&self) -> Point2 {
        Point2 {
            kernel_point_2: self.kernel_arc_segment_2.source(),
        }
    }

    pub fn source_radian(&self) -> f64 {
        self.kernel_arc_segment_2.source_radian()
    }

    pub fn target(&self) -> Point2 {
        Point2 {
            kernel_point_2: self.kernel_arc_segment_2.target(),
        }
    }

    pub fn target_radian(&self) -> f64 {
        self.kernel_arc_segment_2.target_radian()
    }

    pub fn center(&self) -> Point2 {
        Point2 {
            kernel_point_2: self.kernel_arc_segment_2.center(),
        }
    }

    pub fn radius(&self) -> f64 {
        self.kernel_arc_segment_2.radius()
    }
}
