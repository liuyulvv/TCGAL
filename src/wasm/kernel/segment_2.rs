use crate::{
    kernel::util_enum::Segment2Type,
    wasm::kernel::{
        arc_segment_2::ArcSegment2, circle_segment_2::CircleSegment2, line_segment_2::LineSegment2,
    },
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Segment2 {
    pub segment_type: Segment2Type,
    pub line_segment_2: Option<LineSegment2>,
    pub circle_segment_2: Option<CircleSegment2>,
    pub arc_segment_2: Option<ArcSegment2>,
}

pub enum Segment2Value {
    LineSegment2(LineSegment2),
    CircleSegment2(CircleSegment2),
    ArcSegment2(ArcSegment2),
}

#[wasm_bindgen]
impl Segment2 {
    #[wasm_bindgen(constructor)]
    pub fn new(
        segment_type: Segment2Type,
        line_segment_2: Option<LineSegment2>,
        circle_segment_2: Option<CircleSegment2>,
        arc_segment_2: Option<ArcSegment2>,
    ) -> Segment2 {
        Segment2 {
            segment_type,
            line_segment_2,
            circle_segment_2,
            arc_segment_2,
        }
    }
}

pub fn get_segment_value(segment: &Segment2) -> Segment2Value {
    match segment.segment_type {
        Segment2Type::LineSegment2 => Segment2Value::LineSegment2(segment.line_segment_2.unwrap()),
        Segment2Type::CircleSegment2 => {
            Segment2Value::CircleSegment2(segment.circle_segment_2.unwrap())
        }
        Segment2Type::ArcSegment2 => Segment2Value::ArcSegment2(segment.arc_segment_2.unwrap()),
    }
}
