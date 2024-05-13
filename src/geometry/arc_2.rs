use super::circle_2::Circle2;

pub struct Arc2 {
    pub circle: Circle2,
    pub start_angle: f64,
    pub end_angle: f64,
}

impl Arc2 {
    pub fn new(circle: Circle2, start_angle: f64, end_angle: f64) -> Self {
        Self {
            circle,
            start_angle,
            end_angle,
        }
    }
}

impl Default for Arc2 {
    fn default() -> Self {
        Self {
            circle: Circle2::default(),
            start_angle: 0.0,
            end_angle: std::f64::consts::PI,
        }
    }
}
