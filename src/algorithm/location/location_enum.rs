#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2Segment2Location {
    On,
    Left,
    Right,
    Collinear,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2Circle2Location {
    On,
    Inside,
    Outside,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2Polygon2Location {
    On,
    Inside,
    Outside,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2Triangle2Location {
    On,
    Inside,
    Outside,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2Ray2Location {
    On,
    Left,
    Right,
    Collinear,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2ArcSegment2Location {
    On,
    NotOn,
}
