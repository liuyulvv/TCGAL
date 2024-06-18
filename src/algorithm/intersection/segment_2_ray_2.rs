use crate::{
    algorithm::location::{
        location_enum::Point2Ray2Location,
        point_2_ray_2::{is_point_2_on_ray_2, locate_point_2_ray_2},
    },
    kernel::{line_segment_2::LineSegment2, number_type::NumberType, point_2::Point2, ray_2::Ray2},
};

pub fn is_segment_2_ray_2_intersected<T: NumberType>(
    line_segment: &LineSegment2<T>,
    ray_segment: &Ray2<T>,
) -> bool {
    let v1 = line_segment.target() - ray_segment.origin();
    let v2 = line_segment.source() - ray_segment.origin();
    let cross1 = ray_segment.direction().cross(&v1);
    let cross2 = ray_segment.direction().cross(&v2);
    let eps = T::default_eps();
    cross1 * cross2 <= eps && cross1 >= -eps
}

pub fn segment_2_ray_2_intersection<T: NumberType>(
    line_segment: &LineSegment2<T>,
    ray_segment: &Ray2<T>,
) -> Vec<Point2<T>> {
    let mut result = Vec::new();
    let source = line_segment.source();

    let source_relation = locate_point_2_ray_2(&source, ray_segment);
    match source_relation {
        Point2Ray2Location::On => {
            result.push(source);
        }
        _ => {}
    }

    let target = line_segment.target();
    let target_relation = locate_point_2_ray_2(&target, ray_segment);
    match target_relation {
        Point2Ray2Location::On => {
            result.push(target);
            return result;
        }
        Point2Ray2Location::Collinear => {
            return result;
        }
        _ => {
            if target_relation == source_relation {
                return result;
            } else {
                match source_relation {
                    Point2Ray2Location::On => {
                        return result;
                    }
                    Point2Ray2Location::Collinear => {
                        return result;
                    }
                    _ => {
                        let v1 = target - source;
                        let v2 = ray_segment.origin() - source;
                        let v3 = ray_segment.origin() - target;
                        let cross1 = v1.cross(&v2);
                        let cross2 = v1.cross(&v3);
                        if cross1.equals(cross2) {
                            let point = source + target;
                            let point = point / T::from_f64(2.0);
                            let point = Point2::new(point.x(), point.y());
                            if is_point_2_on_ray_2(&point, ray_segment) {
                                result.push(point);
                            }
                            return result;
                        } else {
                            let t = cross1 / (cross1 - cross2);
                            let point =
                                ray_segment.origin().get_vector() + ray_segment.direction() * t;
                            result.push(Point2::new(point.x(), point.y()));
                        }
                        return result;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::{point_2::Point2, vector_2::Vector2};

    #[test]
    fn test_is_segment_2_ray_2_intersected() {
        let line_segment = LineSegment2::new(Point2::new(0.0, 0.0), Point2::new(2.0, 0.0));

        let ray_segment = Ray2::new(Point2::new(1.0, 1.0), Vector2::new(0.0, -1.0));
        assert_eq!(
            is_segment_2_ray_2_intersected(&line_segment, &ray_segment),
            true
        );

        let ray_segment = Ray2::new(Point2::new(1.0, 1.0), Vector2::new(0.0, 1.0));
        assert_eq!(
            is_segment_2_ray_2_intersected(&line_segment, &ray_segment),
            false
        );

        let ray_segment = Ray2::new(Point2::new(1.0, 1.0), Vector2::new(1.0, 0.0));
        assert_eq!(
            is_segment_2_ray_2_intersected(&line_segment, &ray_segment),
            false
        );

        let ray_segment = Ray2::new(Point2::new(0.0, 0.0), Vector2::new(1.0, 0.0));
        assert_eq!(
            is_segment_2_ray_2_intersected(&line_segment, &ray_segment),
            true
        );
    }

    #[test]
    fn test_segment_2_ray_2_intersection() {
        let line_segment = LineSegment2::new(Point2::new(0.0, 0.0), Point2::new(2.0, 0.0));

        let ray_segment = Ray2::new(Point2::new(1.0, 1.0), Vector2::new(0.0, -1.0));
        let result = segment_2_ray_2_intersection(&line_segment, &ray_segment);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Point2::new(1.0, 0.0));

        let ray_segment = Ray2::new(Point2::new(1.0, 1.0), Vector2::new(0.0, 1.0));
        let result = segment_2_ray_2_intersection(&line_segment, &ray_segment);
        assert_eq!(result.len(), 0);

        let ray_segment = Ray2::new(Point2::new(1.0, 1.0), Vector2::new(1.0, 0.0));
        let result = segment_2_ray_2_intersection(&line_segment, &ray_segment);
        assert_eq!(result.len(), 0);

        let ray_segment = Ray2::new(Point2::new(0.0, 0.0), Vector2::new(1.0, 0.0));
        let result = segment_2_ray_2_intersection(&line_segment, &ray_segment);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Point2::new(0.0, 0.0));
        assert_eq!(result[1], Point2::new(2.0, 0.0));
    }
}
