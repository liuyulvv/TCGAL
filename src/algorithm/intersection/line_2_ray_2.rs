use crate::{
    algorithm::location::{location_enum::Point2Ray2Location, point_2_ray_2::locate_point_2_ray_2},
    kernel::{line_2::Line2, number_type::NumberType, point_2::Point2, ray_2::Ray2},
};

use super::line_2_line_2::line_2_line_2_intersection;

pub fn is_line_2_ray_2_intersected<T: NumberType>(line: &Line2<T>, ray: &Ray2<T>) -> bool {
    line_2_ray_2_intersection(line, ray).is_some()
}

pub fn line_2_ray_2_intersection<T: NumberType>(
    line: &Line2<T>,
    ray: &Ray2<T>,
) -> Option<Point2<T>> {
    let origin = ray.origin();
    let direction = ray.direction();
    let target = direction + origin.get_vector();
    let target = Point2::new(target.x(), target.y());
    let line_ray = Line2::from_points(&origin, &target);
    let intersection = line_2_line_2_intersection(&line, &line_ray);
    match intersection {
        Some(point) => match locate_point_2_ray_2(&point, ray) {
            Point2Ray2Location::On => Some(point),
            _ => None,
        },
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::kernel::vector_2::Vector2;

    use super::*;

    #[test]
    fn test_line_2_ray_2_is_intersected() {
        let line = Line2::from_points(&Point2::new(0.0, 0.0), &Point2::new(1.0, 1.0));
        let ray = Ray2::new(Point2::new(0.0, 0.0), Vector2::new(1.0, 1.0));
        assert_eq!(is_line_2_ray_2_intersected(&line, &ray), false);

        let line = Line2::from_points(&Point2::new(0.0, 0.0), &Point2::new(1.0, 1.0));
        let ray = Ray2::new(Point2::new(0.0, 0.0), Vector2::new(-1.0, -1.0));
        assert_eq!(is_line_2_ray_2_intersected(&line, &ray), false);

        let line = Line2::from_points(&Point2::new(0.0, 0.0), &Point2::new(1.0, 1.0));
        let ray = Ray2::new(Point2::new(0.0, 0.0), Vector2::new(1.0, 0.0));
        assert_eq!(is_line_2_ray_2_intersected(&line, &ray), true);

        let line = Line2::from_points(&Point2::new(0.0, 0.0), &Point2::new(1.0, 1.0));
        let ray = Ray2::new(Point2::new(0.0, 0.0), Vector2::new(0.0, 1.0));
        assert_eq!(is_line_2_ray_2_intersected(&line, &ray), true);
    }

    #[test]
    fn test_line_2_ray_2_intersection() {
        let line = Line2::from_points(&Point2::new(0.0, 0.0), &Point2::new(1.0, 1.0));
        let ray = Ray2::new(Point2::new(0.0, 0.0), Vector2::new(1.0, 1.0));
        assert_eq!(line_2_ray_2_intersection(&line, &ray), None);

        let line = Line2::from_points(&Point2::new(0.0, 0.0), &Point2::new(1.0, 1.0));
        let ray = Ray2::new(Point2::new(0.0, 0.0), Vector2::new(-1.0, -1.0));
        assert_eq!(line_2_ray_2_intersection(&line, &ray), None);

        let line = Line2::from_points(&Point2::new(0.0, 0.0), &Point2::new(1.0, 1.0));
        let ray = Ray2::new(Point2::new(0.0, 0.0), Vector2::new(1.0, 0.0));
        assert_eq!(
            line_2_ray_2_intersection(&line, &ray),
            Some(Point2::new(0.0, 0.0))
        );

        let line = Line2::from_points(&Point2::new(0.0, 0.0), &Point2::new(1.0, 1.0));
        let ray = Ray2::new(Point2::new(0.0, 0.0), Vector2::new(0.0, 1.0));
        assert_eq!(
            line_2_ray_2_intersection(&line, &ray),
            Some(Point2::new(0.0, 0.0))
        );
    }
}
