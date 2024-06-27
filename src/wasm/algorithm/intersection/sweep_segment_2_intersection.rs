use crate::{
    algorithm::intersection::sweep_segment_2_intersection::SweepSegment2Intersection as KernelSweepSegment2Intersection,
    wasm::kernel::{
        point_2::Point2,
        segment_2::{get_segment_value, Segment2, Segment2Value},
    },
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SweepSegment2Intersection {
    #[wasm_bindgen(skip)]
    pub kernel_sweep_segment_2_intersection: KernelSweepSegment2Intersection<f64>,
}

#[wasm_bindgen]
impl SweepSegment2Intersection {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            kernel_sweep_segment_2_intersection: KernelSweepSegment2Intersection::new(),
        }
    }

    pub fn push_segment(&mut self, segment_2: &Segment2) {
        let segment = get_segment_value(segment_2);
        match segment {
            Segment2Value::LineSegment2(segment) => {
                self.kernel_sweep_segment_2_intersection
                    .push_segment(&segment.kernel_line_segment_2);
            }
            Segment2Value::CircleSegment2(segment) => {
                self.kernel_sweep_segment_2_intersection
                    .push_segment(&segment.kernel_circle_segment_2);
            }
            Segment2Value::ArcSegment2(segment) => {
                self.kernel_sweep_segment_2_intersection
                    .push_segment(&segment.kernel_arc_segment_2);
            }
        }
    }

    pub fn intersection(&mut self) -> Vec<Point2> {
        self.kernel_sweep_segment_2_intersection
            .intersection()
            .into_iter()
            .map(|point| Point2 {
                kernel_point_2: point,
            })
            .collect()
    }
}
