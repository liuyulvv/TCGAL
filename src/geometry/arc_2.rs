use super::circle_2::Circle2;

pub struct Arc2 {
    pub circle: Circle2,
    pub start_radian: f64,
    pub end_radian: f64,
}

impl Arc2 {
    pub fn new(circle: Circle2, start_radian: f64, end_radian: f64) -> Self {
        Self {
            circle,
            start_radian,
            end_radian,
        }
    }
}

impl Default for Arc2 {
    fn default() -> Self {
        Self {
            circle: Circle2::default(),
            start_radian: 0.0,
            end_radian: std::f64::consts::PI,
        }
    }
}
