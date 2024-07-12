#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Clockwise,
    CounterClockwise,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnDirection {
    Left,
    Right,
    Collinear,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edge2Type {
    Segment,
    Arc,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Segment2Type {
    LineSegment2,
    CircleSegment2,
    ArcSegment2,
}
