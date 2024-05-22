use crate::{
    kernel::base_kernel::{base_circle_2::BaseCircle2, base_vector_2::BaseVector2},
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum Point2Circle2Location {
    On,
    Inside,
    Outside,
}

pub fn is_point_2_on_circle_2<NT, T>(point_2: &T::Point2, circle_2: &T) -> bool
where
    NT: BaseNumberTypeTrait,
    T: BaseCircle2<NT>,
{
    let center = circle_2.center();
    let radius = circle_2.radius();
    let vec_center_point = *point_2 - center;
    let distance = vec_center_point.length();
    let eps = NT::default_eps();
    (distance - radius).abs() < eps
}

pub fn locate_point_2_circle_2<NT, T>(point_2: &T::Point2, circle_2: &T) -> Point2Circle2Location
where
    NT: BaseNumberTypeTrait,
    T: BaseCircle2<NT>,
{
    let center = circle_2.center();
    let radius = circle_2.radius();
    let vec_center_point = *point_2 - center;
    let distance = vec_center_point.length();
    let eps = NT::default_eps();
    if (distance - radius).abs() < eps {
        Point2Circle2Location::On
    } else if distance < radius {
        Point2Circle2Location::Inside
    } else {
        Point2Circle2Location::Outside
    }
}

#[cfg(test)]
mod tests {
    use crate::kernel::{
        base_kernel::base_point_2::BasePoint2,
        simple_cartesian::{circle_2::Circle2, point_2::Point2},
    };

    use super::*;

    #[test]
    fn test_is_point_2_on_circle_2() {
        let circle_2 = Circle2::new(Point2::new(0.0, 0.0), 10.0);

        let point_2 = Point2::new(0.0, 10.0);
        assert_eq!(is_point_2_on_circle_2(&point_2, &circle_2), true);

        let point_2 = Point2::new(0.0, 10.1);
        assert_eq!(is_point_2_on_circle_2(&point_2, &circle_2), false);

        let point_2 = Point2::new(0.0, 9.9);
        assert_eq!(is_point_2_on_circle_2(&point_2, &circle_2), false);

        let point_2 = Point2::new(0.0, 0.0);
        assert_eq!(is_point_2_on_circle_2(&point_2, &circle_2), false);

        let point_2 = Point2::new(5.0 * f64::sqrt(2.0), 5.0 * f64::sqrt(2.0));
        assert_eq!(is_point_2_on_circle_2(&point_2, &circle_2), true);
    }

    #[test]
    fn test_locate_point_2_circle_2() {
        let circle_2 = Circle2::new(Point2::new(0.0, 0.0), 10.0);

        let point_2 = Point2::new(0.0, 10.0);
        assert_eq!(
            locate_point_2_circle_2(&point_2, &circle_2),
            Point2Circle2Location::On
        );

        let point_2 = Point2::new(0.0, 10.1);
        assert_eq!(
            locate_point_2_circle_2(&point_2, &circle_2),
            Point2Circle2Location::Outside
        );

        let point_2 = Point2::new(0.0, 9.9);
        assert_eq!(
            locate_point_2_circle_2(&point_2, &circle_2),
            Point2Circle2Location::Inside
        );

        let point_2 = Point2::new(0.0, 0.0);
        assert_eq!(
            locate_point_2_circle_2(&point_2, &circle_2),
            Point2Circle2Location::Inside
        );

        let point_2 = Point2::new(5.0 * f64::sqrt(2.0), 5.0 * f64::sqrt(2.0));
        assert_eq!(
            locate_point_2_circle_2(&point_2, &circle_2),
            Point2Circle2Location::On
        );
    }
}
