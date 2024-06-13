use crate::{
    algorithm::location::point_2_segment_2::locate_point_2_segment_2,
    kernel::{number_type::NumberType, point_2::Point2, triangle_2::Triangle2},
};

use super::location_enum::{Point2Segment2Location, Point2Triangle2Location};

pub fn locate_point_2_triangle_2<T: NumberType>(
    point: &Point2<T>,
    triangle: &Triangle2<T>,
) -> Point2Triangle2Location {
    let edges = triangle.edges();
    let mut location = None;
    for edge in &edges {
        let edge_location = locate_point_2_segment_2(point, edge);
        match edge_location {
            Point2Segment2Location::On => return Point2Triangle2Location::On,
            Point2Segment2Location::Left => match location {
                Some(Point2Segment2Location::Right) => {
                    return Point2Triangle2Location::Outside;
                }
                None => {
                    location = Some(Point2Segment2Location::Left);
                }
                _ => {}
            },
            Point2Segment2Location::Right => match location {
                Some(Point2Segment2Location::Left) => {
                    return Point2Triangle2Location::Outside;
                }
                None => {
                    location = Some(Point2Segment2Location::Right);
                }
                _ => {}
            },
            _ => {
                return Point2Triangle2Location::Outside;
            }
        }
    }
    return Point2Triangle2Location::Inside;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locate_point_2_triangle_2() {
        let point = Point2::new(1.0, 1.0);
        let triangle = Triangle2::new(
            Point2::new(0.0, 0.0),
            Point2::new(2.0, 0.0),
            Point2::new(0.0, 2.0),
        );
        assert_eq!(
            locate_point_2_triangle_2(&point, &triangle),
            Point2Triangle2Location::On
        );

        let point = Point2::new(3.0, 3.0);
        assert_eq!(
            locate_point_2_triangle_2(&point, &triangle),
            Point2Triangle2Location::Outside
        );

        let point = Point2::new(1.0, 0.5);
        assert_eq!(
            locate_point_2_triangle_2(&point, &triangle),
            Point2Triangle2Location::Inside
        );
    }
}
