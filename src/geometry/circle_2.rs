use super::point_2::Point2;

pub struct Circle2 {
    pub center: Point2,
    pub radius: f64,
}

impl Circle2 {
    pub fn new(center: Point2, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn length(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Default for Circle2 {
    fn default() -> Self {
        Self {
            center: Point2::new(0.0, 0.0),
            radius: 1.0,
        }
    }
}
