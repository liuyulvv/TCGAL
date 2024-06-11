use super::{number_type::NumberType, point_2::Point2, vector_2::Vector2};

pub struct Ray2<T: NumberType> {
    origin: Point2<T>,
    direction: Vector2<T>,
}

impl<T: NumberType> Ray2<T> {
    pub fn new(origin: Point2<T>, direction: Vector2<T>) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn origin(&self) -> Point2<T> {
        self.origin.clone()
    }

    pub fn direction(&self) -> Vector2<T> {
        self.direction.clone()
    }
}
