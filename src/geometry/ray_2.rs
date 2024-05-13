use crate::math::vector_2::Vector2;

use super::point_2::Point2;

pub struct Ray2 {
    pub origin: Point2,
    pub direction: Vector2,
}

impl Ray2 {
    pub fn new(origin: Point2, direction: Vector2) -> Self {
        Self { origin, direction }
    }
}

impl Default for Ray2 {
    fn default() -> Self {
        Self {
            origin: Point2::new(0.0, 0.0),
            direction: Vector2::new(1.0, 0.0),
        }
    }
}
