use crate::kernel::{
    number_type::NumberType, point_2::Point2, segment_2::Segment2, vector_2::Vector2,
};

pub fn is_circle_segment_2_circle_segment_2_intersected<T: NumberType>(
    circle_segment_a: &impl Segment2<T>,
    circle_segment_b: &impl Segment2<T>,
) -> bool {
    let center_a = circle_segment_a.center();
    let center_b = circle_segment_b.center();
    let radius_a = circle_segment_a.radius();
    let radius_b = circle_segment_b.radius();
    let distance = center_a.distance(&center_b);
    let radius_sum = radius_a + radius_b;
    let radius_dif = radius_a - radius_b;
    let radius_dif = radius_dif.abs();
    if distance.equals(T::zero()) {
        return false;
    } else if distance.equals(radius_sum) || distance.equals(radius_dif) {
        return true;
    } else if distance < radius_sum && distance > radius_dif {
        return true;
    } else {
        return false;
    }
}

pub fn circle_segment_2_circle_segment_2_intersection<T: NumberType>(
    circle_segment_a: &impl Segment2<T>,
    circle_segment_b: &impl Segment2<T>,
) -> Vec<Point2<T>> {
    let center_a = circle_segment_a.center();
    let center_b = circle_segment_b.center();
    let radius_a = circle_segment_a.radius();
    let radius_b = circle_segment_b.radius();
    let distance = center_a.distance(&center_b);
    let radius_sum = radius_a + radius_b;
    let radius_dif = radius_a - radius_b;
    let radius_dif = radius_dif.abs();
    let mut result = Vec::new();
    if distance.equals(T::zero()) {
        return result;
    } else if distance.equals(radius_sum) {
        let vector = center_b - center_a;
        let vector = vector.normalize();
        let point = center_a.get_vector() + vector * radius_a;
        result.push(Point2::new(point.x(), point.y()));
        return result;
    } else if distance.equals(radius_dif) {
        if radius_a > radius_b {
            let vector = center_b - center_a;
            let vector = vector.normalize();
            let point = center_b.get_vector() + vector * radius_b;
            result.push(Point2::new(point.x(), point.y()));
        } else {
            let vector = center_a - center_b;
            let vector = vector.normalize();
            let point = center_a.get_vector() + vector * radius_a;
            result.push(Point2::new(point.x(), point.y()));
        }
        return result;
    } else if distance < radius_sum && distance > radius_dif {
        let a = (radius_a * radius_a + distance * distance - radius_b * radius_b)
            / (T::from_f64(2.0) * distance);
        let h = (radius_a * radius_a - a * a).sqrt();
        let vector = center_b - center_a;
        let vector = vector.normalize();
        let x = center_a.get_vector() + vector * a;
        let vector = Vector2::new(-vector.y(), vector.x());
        let y = x + vector * h;
        result.push(Point2::new(y.x(), y.y()));
        let y = x - vector * h;
        result.push(Point2::new(y.x(), y.y()));
        return result;
    } else {
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::circle_segment_2::CircleSegment2;
    use crate::kernel::point_2::Point2;

    #[test]
    fn test_is_circle_segment_2_circle_segment_2_intersected() {
        let circle_segment_a = CircleSegment2::new(Point2::new(0.0, 0.0), 5.0);

        let circle_segment_b = CircleSegment2::new(Point2::new(0.0, 0.0), 5.0);
        assert_eq!(
            is_circle_segment_2_circle_segment_2_intersected(&circle_segment_a, &circle_segment_b),
            false
        );

        let circle_segment_b = CircleSegment2::new(Point2::new(6.0, 0.0), 4.0);
        assert_eq!(
            is_circle_segment_2_circle_segment_2_intersected(&circle_segment_a, &circle_segment_b),
            true
        );

        let circle_segment_b = CircleSegment2::new(Point2::new(10.0, 0.0), 4.0);
        assert_eq!(
            is_circle_segment_2_circle_segment_2_intersected(&circle_segment_a, &circle_segment_b),
            false
        );

        let circle_segment_b = CircleSegment2::new(Point2::new(10.0, 0.0), 5.0);
        assert_eq!(
            is_circle_segment_2_circle_segment_2_intersected(&circle_segment_a, &circle_segment_b),
            true
        );

        let circle_segment_b = CircleSegment2::new(Point2::new(2.0, 0.0), 2.0);
        assert_eq!(
            is_circle_segment_2_circle_segment_2_intersected(&circle_segment_a, &circle_segment_b),
            false
        );

        let circle_segment_b = CircleSegment2::new(Point2::new(2.0, 0.0), 3.0);
        assert_eq!(
            is_circle_segment_2_circle_segment_2_intersected(&circle_segment_a, &circle_segment_b),
            true
        );
    }

    #[test]
    fn test_circle_segment_2_circle_segment_2_intersection() {
        let circle_segment_a = CircleSegment2::new(Point2::new(0.0, 0.0), 5.0);

        let circle_segment_b = CircleSegment2::new(Point2::new(0.0, 0.0), 5.0);
        assert_eq!(
            circle_segment_2_circle_segment_2_intersection(&circle_segment_a, &circle_segment_b),
            Vec::new()
        );

        let circle_segment_b = CircleSegment2::new(Point2::new(6.0, 0.0), 4.0);
        assert_eq!(
            circle_segment_2_circle_segment_2_intersection(&circle_segment_a, &circle_segment_b),
            vec![
                Point2::new(3.75, 3.30718913883),
                Point2::new(3.75, -3.30718913883)
            ]
        );

        let circle_segment_b = CircleSegment2::new(Point2::new(10.0, 0.0), 4.0);
        assert_eq!(
            circle_segment_2_circle_segment_2_intersection(&circle_segment_a, &circle_segment_b),
            Vec::new()
        );

        let circle_segment_b = CircleSegment2::new(Point2::new(10.0, 0.0), 5.0);
        assert_eq!(
            circle_segment_2_circle_segment_2_intersection(&circle_segment_a, &circle_segment_b),
            vec![Point2::new(5.0, 0.0)]
        );

        let circle_segment_b = CircleSegment2::new(Point2::new(2.0, 0.0), 2.0);
        assert_eq!(
            circle_segment_2_circle_segment_2_intersection(&circle_segment_a, &circle_segment_b),
            Vec::new()
        );

        let circle_segment_b = CircleSegment2::new(Point2::new(2.0, 0.0), 3.0);
        assert_eq!(
            circle_segment_2_circle_segment_2_intersection(&circle_segment_a, &circle_segment_b),
            vec![Point2::new(5.0, 0.0)]
        );
    }
}
