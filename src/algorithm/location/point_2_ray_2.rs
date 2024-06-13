use crate::kernel::{number_type::NumberType, point_2::Point2, ray_2::Ray2};

use super::location_enum::Point2Ray2Location;

pub fn is_point_2_on_ray_2<T: NumberType>(p: &Point2<T>, r: &Ray2<T>) -> bool {
    if p.equals(&r.origin()) {
        return true;
    }
    let v = *p - r.origin();
    let cross = r.direction().cross(&v);
    let dot = r.direction().dot(&v);
    dot > T::zero() && cross.equals(T::zero())
}

pub fn locate_point_2_ray_2<T: NumberType>(p: &Point2<T>, r: &Ray2<T>) -> Point2Ray2Location {
    if is_point_2_on_ray_2(p, r) {
        return Point2Ray2Location::On;
    }
    let v = *p - r.origin();
    let cross = r.direction().cross(&v);
    if cross > T::default_eps() {
        return Point2Ray2Location::Left;
    } else if cross < -T::default_eps() {
        return Point2Ray2Location::Right;
    }
    return Point2Ray2Location::Collinear;
}

#[cfg(test)]
mod tests {
    use crate::kernel::vector_2::Vector2;

    use super::*;

    #[test]
    fn test_is_point_2_on_ray_2() {
        let r = Ray2::new(Point2::new(0.0, 0.0), Vector2::new(1.0, 0.0));
        let p = Point2::new(1.0, 0.0);
        assert_eq!(is_point_2_on_ray_2(&p, &r), true);

        let p = Point2::new(1.0, 1.0);
        assert_eq!(is_point_2_on_ray_2(&p, &r), false);

        let p = Point2::new(0.0, 0.0);
        assert_eq!(is_point_2_on_ray_2(&p, &r), true);

        let p = Point2::new(-1.0, 0.0);
        assert_eq!(is_point_2_on_ray_2(&p, &r), false);
    }

    #[test]
    fn test_locate_point_2_ray_2() {
        let r = Ray2::new(Point2::new(0.0, 0.0), Vector2::new(1.0, 0.0));
        let p = Point2::new(1.0, 0.0);
        assert_eq!(locate_point_2_ray_2(&p, &r), Point2Ray2Location::On);

        let p = Point2::new(1.0, 1.0);
        assert_eq!(locate_point_2_ray_2(&p, &r), Point2Ray2Location::Left);

        let p = Point2::new(0.0, 0.0);
        assert_eq!(locate_point_2_ray_2(&p, &r), Point2Ray2Location::On);

        let p = Point2::new(-1.0, 0.0);
        assert_eq!(locate_point_2_ray_2(&p, &r), Point2Ray2Location::Collinear);

        let p = Point2::new(1.0, -1.0);
        assert_eq!(locate_point_2_ray_2(&p, &r), Point2Ray2Location::Right);
    }
}
