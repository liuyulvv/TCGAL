use crate::{
    algorithm::{
        location::{
            location_enum::Point2Circle2Location, point_2_circle_2::locate_point_2_circle_2,
        },
        projection::point_2_segment_2::point_2_project_segment_2,
    },
    kernel::{
        circle_segment_2::CircleSegment2, line_segment_2::LineSegment2, number_type::NumberType,
        point_2::Point2,
    },
};

pub fn is_segment_2_circle_2_intersected<T: NumberType>(
    line_segment: &LineSegment2<T>,
    circle_segment: &CircleSegment2<T>,
) -> bool {
    let source = line_segment.source();
    let target = line_segment.target();
    let center = circle_segment.center();
    let source_relation = locate_point_2_circle_2(&source, circle_segment);
    let target_relation = locate_point_2_circle_2(&target, circle_segment);
    if source_relation == Point2Circle2Location::Inside
        && target_relation == Point2Circle2Location::Inside
    {
        return false;
    } else if source_relation == Point2Circle2Location::On
        || target_relation == Point2Circle2Location::On
    {
        return true;
    } else {
        let projection_point = point_2_project_segment_2(&center, &line_segment);
        match projection_point {
            Some(projection_point) => {
                let projection_relation =
                    locate_point_2_circle_2(&projection_point, circle_segment);
                if projection_relation == Point2Circle2Location::On
                    || projection_relation == Point2Circle2Location::Inside
                {
                    return true;
                } else {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }
}

pub fn segment_2_circle_2_intersection<T: NumberType>(
    line_segment: &LineSegment2<T>,
    circle_segment: &CircleSegment2<T>,
) -> Vec<Point2<T>> {
    let mut result = Vec::new();
    let source = line_segment.source();
    let target = line_segment.target();
    let center = circle_segment.center();
    let source_relation = locate_point_2_circle_2(&source, circle_segment);
    let target_relation = locate_point_2_circle_2(&target, circle_segment);
    if source_relation == Point2Circle2Location::Inside
        && target_relation == Point2Circle2Location::Inside
    {
        return result;
    } else if source_relation == Point2Circle2Location::On
        && target_relation == Point2Circle2Location::On
    {
        result.push(source);
        result.push(target);
        return result;
    } else {
        if source_relation == Point2Circle2Location::Inside
            || target_relation == Point2Circle2Location::Inside
        {
            let mut inside_point = source;
            let mut outside_point = target;
            if target_relation == Point2Circle2Location::Inside {
                inside_point = target;
                outside_point = source;
            }
            let v = outside_point - inside_point;
            let w = circle_segment.center() - inside_point;
            let b = w.dot(&v) / v.dot(&v);
            let t = v * b;
            let projection_point = Point2::new(t.x(), t.y());
            let projection_point = inside_point + projection_point;
            let projection_point = Point2::new(projection_point.x(), projection_point.y());
            let distance: T = circle_segment.center().distance(&projection_point);
            let offset =
                (circle_segment.radius() * circle_segment.radius() - distance * distance).sqrt();
            let projection_point = projection_point.get_vector() + v.normalize() * offset;
            result.push(Point2::new(projection_point.x(), projection_point.y()));
            return result;
        } else {
            let projection_point = point_2_project_segment_2(&center, &line_segment);
            match projection_point {
                Some(projection_point) => {
                    let projection_relation =
                        locate_point_2_circle_2(&projection_point, circle_segment);
                    if projection_relation == Point2Circle2Location::On {
                        result.push(projection_point);
                        return result;
                    } else if projection_relation == Point2Circle2Location::Inside {
                        let distance = circle_segment.center().distance(&projection_point);
                        let radius = circle_segment.radius();
                        let offset = radius * radius - distance * distance;
                        let offset = offset.sqrt();
                        let vector = (target - source).normalize();
                        let point_a = projection_point.get_vector() + vector * offset;
                        let point_b = projection_point.get_vector() - vector * offset;
                        result.push(Point2::new(point_a.x(), point_a.y()));
                        result.push(Point2::new(point_b.x(), point_b.y()));
                        return result;
                    } else {
                        return result;
                    }
                }
                None => {
                    if source_relation == Point2Circle2Location::On {
                        result.push(source);
                    } else if target_relation == Point2Circle2Location::On {
                        result.push(target);
                    }
                    return result;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::kernel::point_2::Point2;

    use super::*;

    #[test]
    fn is_intersected() {
        let segment_2 = LineSegment2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0));
        let circle_2 = CircleSegment2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            is_segment_2_circle_2_intersected(&segment_2, &circle_2),
            true
        );

        let segment_2 = LineSegment2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0));
        let circle_2 = CircleSegment2::new(Point2::new(0.0, 0.0), 2.0);
        assert_eq!(
            is_segment_2_circle_2_intersected(&segment_2, &circle_2),
            false
        );

        let segment_2 = LineSegment2::new(
            Point2::new(0.0, 0.0),
            Point2::new(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let circle_2 = CircleSegment2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            is_segment_2_circle_2_intersected(&segment_2, &circle_2),
            true
        );

        let segment_2 = LineSegment2::new(
            Point2::new(f64::sqrt(2.0) / 2.0 + 0.1, f64::sqrt(2.0) / 2.0),
            Point2::new(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let circle_2 = CircleSegment2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            is_segment_2_circle_2_intersected(&segment_2, &circle_2),
            true
        );

        let segment_2 = LineSegment2::new(
            Point2::new(f64::sqrt(2.0) / 2.0 + 0.1, f64::sqrt(2.0) / 2.0),
            Point2::new(3.0, 3.0),
        );
        let circle_2 = CircleSegment2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            is_segment_2_circle_2_intersected(&segment_2, &circle_2),
            false
        );

        let segment_2 = LineSegment2::new(Point2::new(-2.0, 0.0), Point2::new(2.0, 0.0));
        let circle_2 = CircleSegment2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            is_segment_2_circle_2_intersected(&segment_2, &circle_2),
            true
        );
    }

    #[test]
    fn intersection() {
        let segment_2 = LineSegment2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0));
        let circle_2 = CircleSegment2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            segment_2_circle_2_intersection(&segment_2, &circle_2),
            vec!(Point2::new(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0))
        );

        let segment_2 = LineSegment2::new(
            Point2::new(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
            Point2::new(f64::sqrt(2.0) / -2.0, f64::sqrt(2.0) / 2.0),
        );
        let circle_2 = CircleSegment2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            segment_2_circle_2_intersection(&segment_2, &circle_2),
            vec!(
                Point2::new(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
                Point2::new(f64::sqrt(2.0) / -2.0, f64::sqrt(2.0) / 2.0)
            )
        );

        let segment_2 = LineSegment2::new(
            Point2::new(10.0, f64::sqrt(2.0) / 2.0),
            Point2::new(-10.0, f64::sqrt(2.0) / 2.0),
        );
        let circle_2 = CircleSegment2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            segment_2_circle_2_intersection(&segment_2, &circle_2),
            vec!(
                Point2::new(f64::sqrt(2.0) / -2.0, f64::sqrt(2.0) / 2.0),
                Point2::new(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
            )
        );

        let segment_2 = LineSegment2::new(Point2::new(10.0, 10.0), Point2::new(-10.0, -10.0));
        let circle_2 = CircleSegment2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            segment_2_circle_2_intersection(&segment_2, &circle_2),
            vec!(
                Point2::new(f64::sqrt(2.0) / -2.0, f64::sqrt(2.0) / -2.0),
                Point2::new(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
            )
        );

        let segment_2 = LineSegment2::new(Point2::new(4.0, 3.0), Point2::new(10.0, 3.0));
        let circle_2 = CircleSegment2::new(Point2::new(0.0, 0.0), 5.0);
        assert_eq!(
            segment_2_circle_2_intersection(&segment_2, &circle_2),
            vec!(Point2::new(4.0, 3.0))
        );

        let segment_2 = LineSegment2::new(Point2::new(4.0, 3.0), Point2::new(-10.0, 3.0));
        let circle_2 = CircleSegment2::new(Point2::new(0.0, 0.0), 5.0);
        assert_eq!(
            segment_2_circle_2_intersection(&segment_2, &circle_2),
            vec!(Point2::new(-4.0, 3.0), Point2::new(4.0, 3.0),)
        );
    }
}
