use crate::kernel::vector_2::Vector2 as KernelVector2;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Vector2 {
    #[wasm_bindgen(skip)]
    pub kernel_vector_2: KernelVector2<f64>,
}

#[wasm_bindgen]
impl Vector2 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            kernel_vector_2: KernelVector2::new(x, y),
        }
    }

    pub fn x(&self) -> f64 {
        self.kernel_vector_2.x()
    }

    pub fn y(&self) -> f64 {
        self.kernel_vector_2.y()
    }

    pub fn length(&self) -> f64 {
        self.kernel_vector_2.length()
    }

    pub fn normalize(&self) -> Self {
        Self {
            kernel_vector_2: self.kernel_vector_2.normalize(),
        }
    }

    pub fn dot(&self, other: &Vector2) -> f64 {
        self.kernel_vector_2.dot(&other.kernel_vector_2)
    }

    pub fn cross(&self, other: &Vector2) -> f64 {
        self.kernel_vector_2.cross(&other.kernel_vector_2)
    }

    pub fn radian_to(&self, other: &Vector2) -> f64 {
        self.kernel_vector_2.radian_to(&other.kernel_vector_2)
    }
}
