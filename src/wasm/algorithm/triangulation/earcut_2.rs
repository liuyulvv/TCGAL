use crate::{
    algorithm::triangulation::earcut_2::earcut_2 as KernelEarcut2,
    wasm::kernel::polygon_2::Polygon2,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Triangle2Indices {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

#[wasm_bindgen]
pub fn earcut(polygon: Polygon2) -> Vec<Triangle2Indices> {
    KernelEarcut2(polygon.kernel_polygon_2)
        .iter()
        .map(|t| Triangle2Indices {
            a: t[0],
            b: t[1],
            c: t[2],
        })
        .collect()
}
