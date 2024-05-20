use crate::{
    kernel::base_kernel::{base_circle_2::BaseCircle2, base_point_2::BasePoint2},
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

use super::point_2::Point2;

#[derive(Debug, Clone, Copy)]
pub struct Circle2<'a, T: BaseNumberTypeTrait> {
    center: &'a Point2<T>,
    radius: T,
}

impl<'a, T: BaseNumberTypeTrait> Circle2<'a, T> {
    pub fn new(center: &'a Point2<T>, radius: T) -> Self {
        Self { center, radius }
    }
}

impl<'a, T: BaseNumberTypeTrait> BaseCircle2<'a, T> for Circle2<'a, T> {
    fn center(&self) -> Box<dyn BasePoint2<T> + 'a> {
        Box::new(*self.center)
    }

    fn radius(&self) -> T {
        self.radius
    }
}
