use crate::kernel::{line_2::Line2, number_type::NumberType, point_2::Point2};

use super::location_enum::Point2Line2Location;

pub fn is_point_2_on_line_2<T: NumberType>(point: &Point2<T>, line: &Line2<T>) -> bool {
    locate_point_2_line_2(point, line) == Point2Line2Location::On
}

pub fn locate_point_2_line_2<T: NumberType>(
    point: &Point2<T>,
    line: &Line2<T>,
) -> Point2Line2Location {
    let a = line.a();
    let b = line.b();
    let c = line.c();
    let x = point.x();
    let y = point.y();
    let result = a * x + b * y + c;
    if result.equals(T::zero()) {
        Point2Line2Location::On
    } else {
        Point2Line2Location::NotOn
    }
}
