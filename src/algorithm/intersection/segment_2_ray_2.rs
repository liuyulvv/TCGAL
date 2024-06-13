use crate::{
    algorithm::location::{
        location_enum::Point2Ray2Location,
        point_2_ray_2::{is_point_2_on_ray_2, locate_point_2_ray_2},
    },
    kernel::{number_type::NumberType, point_2::Point2, ray_2::Ray2, segment_2::Segment2},
};

pub fn is_segment_2_ray_2_intersected<T: NumberType>(s: &Segment2<T>, r: &Ray2<T>) -> bool {
    let v1 = s.target() - r.origin();
    let v2 = s.source() - r.origin();
    let cross1 = r.direction().cross(&v1);
    let cross2 = r.direction().cross(&v2);
    let eps = T::default_eps();
    cross1 * cross2 <= eps && cross1 >= -eps
}

pub fn segment_2_ray_2_intersection<T: NumberType>(s: &Segment2<T>, r: &Ray2<T>) -> Vec<Point2<T>> {
    let mut result = Vec::new();
    let source = s.source();

    let source_relation = locate_point_2_ray_2(&source, r);
    match source_relation {
        Point2Ray2Location::On => {
            result.push(source);
        }
        _ => {}
    }

    let target = s.target();
    let target_relation = locate_point_2_ray_2(&target, r);
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
                        let v2 = r.origin() - source;
                        let v3 = r.origin() - target;
                        let cross1 = v1.cross(&v2);
                        let cross2 = v1.cross(&v3);
                        if cross1.equals(cross2) {
                            let point = source + target;
                            let point = point / T::from_f64(2.0);
                            let point = Point2::new(point.x(), point.y());
                            if is_point_2_on_ray_2(&point, r) {
                                result.push(point);
                            }
                            return result;
                        } else {
                            let t = cross1 / (cross1 - cross2);
                            let point = r.origin().get_vector() + r.direction() * t;
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
        let s = Segment2::new(Point2::new(0.0, 0.0), Point2::new(2.0, 0.0));

        let r = Ray2::new(Point2::new(1.0, 1.0), Vector2::new(0.0, -1.0));
        assert_eq!(is_segment_2_ray_2_intersected(&s, &r), true);

        let r = Ray2::new(Point2::new(1.0, 1.0), Vector2::new(0.0, 1.0));
        assert_eq!(is_segment_2_ray_2_intersected(&s, &r), false);

        let r = Ray2::new(Point2::new(1.0, 1.0), Vector2::new(1.0, 0.0));
        assert_eq!(is_segment_2_ray_2_intersected(&s, &r), false);

        let r = Ray2::new(Point2::new(0.0, 0.0), Vector2::new(1.0, 0.0));
        assert_eq!(is_segment_2_ray_2_intersected(&s, &r), true);
    }

    #[test]
    fn test_segment_2_ray_2_intersection() {
        let s = Segment2::new(Point2::new(0.0, 0.0), Point2::new(2.0, 0.0));

        let r = Ray2::new(Point2::new(1.0, 1.0), Vector2::new(0.0, -1.0));
        let result = segment_2_ray_2_intersection(&s, &r);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Point2::new(1.0, 0.0));

        let r = Ray2::new(Point2::new(1.0, 1.0), Vector2::new(0.0, 1.0));
        let result = segment_2_ray_2_intersection(&s, &r);
        assert_eq!(result.len(), 0);

        let r = Ray2::new(Point2::new(1.0, 1.0), Vector2::new(1.0, 0.0));
        let result = segment_2_ray_2_intersection(&s, &r);
        assert_eq!(result.len(), 0);

        let r = Ray2::new(Point2::new(0.0, 0.0), Vector2::new(1.0, 0.0));
        let result = segment_2_ray_2_intersection(&s, &r);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Point2::new(0.0, 0.0));
        assert_eq!(result[1], Point2::new(2.0, 0.0));
    }
}
