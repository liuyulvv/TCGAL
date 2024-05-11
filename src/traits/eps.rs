#[derive(Debug, Clone, Copy)]
pub struct Eps {
    pub value: f64,
}

impl Eps {
    pub fn new(value: f64) -> Self {
        Eps { value }
    }
}

impl Default for Eps {
    fn default() -> Self {
        Eps { value: 1e-6 }
    }
}
