use crate::kernel::{number_type::NumberType, point_2::Point2, polygon_2::Polygon2};

use super::location_enum::Point2Polygon2Location;

pub fn is_point_2_on_polygon_2<T: NumberType>(point: &Point2<T>, polygon: &Polygon2<T>) -> bool {
    let location = locate_point_2_polygon_2(point, polygon);
    match location {
        Point2Polygon2Location::On => true,
        _ => false,
    }
}

pub fn is_point_2_inside_polygon_2<T: NumberType>(
    point: &Point2<T>,
    polygon: &Polygon2<T>,
) -> bool {
    let location = locate_point_2_polygon_2(point, polygon);
    match location {
        Point2Polygon2Location::Inside => true,
        _ => false,
    }
}

pub fn is_point_2_outside_polygon_2<T: NumberType>(
    point: &Point2<T>,
    polygon: &Polygon2<T>,
) -> bool {
    let location = locate_point_2_polygon_2(point, polygon);
    match location {
        Point2Polygon2Location::Outside => true,
        _ => false,
    }
}

pub fn locate_point_2_polygon_2<T: NumberType>(
    point: &Point2<T>,
    polygon: &Polygon2<T>,
) -> Point2Polygon2Location {
    todo!()
}
