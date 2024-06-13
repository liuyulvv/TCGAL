use crate::kernel::{number_type::NumberType, point_2::Point2, triangle_2::Triangle2};

use super::location_enum::Point2Triangle2Location;

pub fn is_point_2_on_triangle_2<T: NumberType>(point: &Point2<T>, triangle: &Triangle2<T>) -> bool {
    let location = locate_point_2_triangle_2(point, triangle);
    match location {
        Point2Triangle2Location::On => true,
        _ => false,
    }
}

pub fn locate_point_2_triangle_2<T: NumberType>(
    point: &Point2<T>,
    triangle: &Triangle2<T>,
) -> Point2Triangle2Location {
    todo!()
}
