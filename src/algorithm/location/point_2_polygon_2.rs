use crate::kernel::{number_type::NumberType, point_2::Point2, polygon_2::Polygon2};

use super::{
    location_enum::{Point2Polygon2Location, Point2Segment2Location},
    point_2_line_segment_2::locate_point_2_line_segment_2,
};

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
    let edges = polygon.edges();
    let mut location = None;
    for edge in &edges {
        let edge_location = locate_point_2_line_segment_2(point, edge);
        match edge_location {
            Point2Segment2Location::On => return Point2Polygon2Location::On,
            Point2Segment2Location::Left => match location {
                Some(Point2Segment2Location::Right) => {
                    return Point2Polygon2Location::Outside;
                }
                None => {
                    location = Some(Point2Segment2Location::Left);
                }
                _ => {}
            },
            Point2Segment2Location::Right => match location {
                Some(Point2Segment2Location::Left) => {
                    return Point2Polygon2Location::Outside;
                }
                None => {
                    location = Some(Point2Segment2Location::Right);
                }
                _ => {}
            },
            _ => {}
        }
    }
    match location {
        None => return Point2Polygon2Location::Outside,
        _ => {
            return Point2Polygon2Location::Inside;
        }
    }
}
