use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2Segment2Location {
    On,
    Left,
    Right,
    Collinear,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2Circle2Location {
    On,
    Inside,
    Outside,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2Polygon2Location {
    On,
    Inside,
    Outside,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2Triangle2Location {
    On,
    Inside,
    Outside,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2Ray2Location {
    On,
    Left,
    Right,
    Collinear,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2ArcSegment2Location {
    On,
    NotOn,
}
