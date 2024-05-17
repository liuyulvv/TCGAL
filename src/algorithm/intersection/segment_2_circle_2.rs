use std::vec;

use crate::{
    algorithm::{
        location::{
            location_enum::Point2Circle2Relation, point_2_circle_2::point_2_relation_to_circle_2,
            point_2_segment_2::is_point_2_on_segment_2,
        },
        projection::point_2_segment_2::point_2_project_segment_2,
    },
    geometry::{circle_2::Circle2, point_2::Point2, segment_2::Segment2},
    traits::eps::Eps,
};

pub fn is_segment_2_circle_2_intersected(
    segment_2: &Segment2,
    circle_2: &Circle2,
    eps: Option<Eps>,
) -> bool {
    let start_relation = point_2_relation_to_circle_2(&segment_2.start, circle_2, eps);
    let end_relation = point_2_relation_to_circle_2(&segment_2.end, circle_2, eps);
    if start_relation == Point2Circle2Relation::ON || end_relation == Point2Circle2Relation::ON {
        return true;
    }
    if start_relation == Point2Circle2Relation::INSIDE
        && end_relation == Point2Circle2Relation::INSIDE
    {
        return false;
    }
    if start_relation == Point2Circle2Relation::OUTSIDE
        && end_relation == Point2Circle2Relation::OUTSIDE
    {
        let projection = point_2_project_segment_2(&circle_2.center, segment_2, eps);
        match projection {
            Some(projection) => {
                let relation = point_2_relation_to_circle_2(&projection, circle_2, eps);
                return relation == Point2Circle2Relation::ON
                    || relation == Point2Circle2Relation::INSIDE;
            }
            None => {
                return false;
            }
        }
    }
    return true;
}

pub fn segment_2_circle_2_intersection_point_2(
    segment_2: &Segment2,
    circle_2: &Circle2,
    eps: Option<Eps>,
) -> Option<Vec<Point2>> {
    let start_relation = point_2_relation_to_circle_2(&segment_2.start, circle_2, eps);
    let end_relation = point_2_relation_to_circle_2(&segment_2.end, circle_2, eps);
    if start_relation == Point2Circle2Relation::INSIDE
        && end_relation == Point2Circle2Relation::INSIDE
    {
        return None;
    }
    if start_relation == Point2Circle2Relation::ON && end_relation == Point2Circle2Relation::ON {
        let mut result = vec![segment_2.start.clone(), segment_2.end.clone()];
        result.sort_by(|a, b| {
            if a.x < b.x {
                return std::cmp::Ordering::Less;
            } else if a.x > b.x {
                return std::cmp::Ordering::Greater;
            } else {
                if a.y < b.y {
                    return std::cmp::Ordering::Less;
                } else if a.y > b.y {
                    return std::cmp::Ordering::Greater;
                } else {
                    return std::cmp::Ordering::Equal;
                }
            }
        });
        return Some(result);
    }
    let projection = point_2_project_segment_2(&circle_2.center, segment_2, eps);
    if start_relation == Point2Circle2Relation::OUTSIDE
        && end_relation == Point2Circle2Relation::OUTSIDE
    {
        match projection {
            Some(projection) => {
                let relation = point_2_relation_to_circle_2(&projection, circle_2, eps);
                if relation == Point2Circle2Relation::OUTSIDE {
                    return None;
                } else if relation == Point2Circle2Relation::ON {
                    let mut result = vec![];
                    result.push(projection);
                    return Some(result);
                }
            }
            None => {
                return None;
            }
        }
    }
    if projection.is_none() {
        return None;
    }
    let projection_point = projection.unwrap();
    let segment_2_direction = segment_2.end - segment_2.start;
    let segment_2_direction = segment_2_direction.normalize();
    let projection_point_center_distance = circle_2.center.distance_to(&projection_point);
    let offset = f64::sqrt(circle_2.radius.powi(2) - projection_point_center_distance.powi(2));
    let intersection_point_1 =
        projection_point + Point2::from_vector(&(segment_2_direction * offset));
    let intersection_point_2 =
        projection_point + Point2::from_vector(&(segment_2_direction * -offset));
    let mut result = vec![intersection_point_1, intersection_point_2];
    result.retain(|point| {
        let relation = point_2_relation_to_circle_2(point, circle_2, eps);
        let is_on = is_point_2_on_segment_2(point, segment_2, eps);
        return relation == Point2Circle2Relation::ON && is_on;
    });
    result.sort_by(|a, b| {
        if a.x < b.x {
            return std::cmp::Ordering::Less;
        } else if a.x > b.x {
            return std::cmp::Ordering::Greater;
        } else {
            if a.y < b.y {
                return std::cmp::Ordering::Less;
            } else if a.y > b.y {
                return std::cmp::Ordering::Greater;
            } else {
                return std::cmp::Ordering::Equal;
            }
        }
    });
    return Some(result);
}

#[cfg(test)]
mod tests {
    use crate::geometry::point_2::Point2;

    use super::*;

    #[test]
    fn is_intersected() {
        let segment_2 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0));
        let circle_2 = Circle2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            is_segment_2_circle_2_intersected(&segment_2, &circle_2, None),
            true
        );

        let segment_2 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0));
        let circle_2 = Circle2::new(Point2::new(0.0, 0.0), 2.0);
        assert_eq!(
            is_segment_2_circle_2_intersected(&segment_2, &circle_2, None),
            false
        );

        let segment_2 = Segment2::new(
            Point2::new(0.0, 0.0),
            Point2::new(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let circle_2 = Circle2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            is_segment_2_circle_2_intersected(&segment_2, &circle_2, None),
            true
        );

        let segment_2 = Segment2::new(
            Point2::new(f64::sqrt(2.0) / 2.0 + 0.1, f64::sqrt(2.0) / 2.0),
            Point2::new(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let circle_2 = Circle2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            is_segment_2_circle_2_intersected(&segment_2, &circle_2, None),
            true
        );

        let segment_2 = Segment2::new(
            Point2::new(f64::sqrt(2.0) / 2.0 + 0.1, f64::sqrt(2.0) / 2.0),
            Point2::new(3.0, 3.0),
        );
        let circle_2 = Circle2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            is_segment_2_circle_2_intersected(&segment_2, &circle_2, None),
            false
        );

        let segment_2 = Segment2::new(Point2::new(-2.0, 0.0), Point2::new(2.0, 0.0));
        let circle_2 = Circle2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            is_segment_2_circle_2_intersected(&segment_2, &circle_2, None),
            true
        );
    }

    #[test]
    fn intersection() {
        let segment_2 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0));
        let circle_2 = Circle2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            segment_2_circle_2_intersection_point_2(&segment_2, &circle_2, None),
            Some(vec![Point2::new(
                f64::sqrt(2.0) / 2.0,
                f64::sqrt(2.0) / 2.0
            ),])
        );

        let segment_2 = Segment2::new(
            Point2::new(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
            Point2::new(f64::sqrt(2.0) / -2.0, f64::sqrt(2.0) / 2.0),
        );
        let circle_2 = Circle2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            segment_2_circle_2_intersection_point_2(&segment_2, &circle_2, None),
            Some(vec![
                Point2::new(f64::sqrt(2.0) / -2.0, f64::sqrt(2.0) / 2.0),
                Point2::new(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
            ])
        );

        let segment_2 = Segment2::new(
            Point2::new(10.0, f64::sqrt(2.0) / 2.0),
            Point2::new(-10.0, f64::sqrt(2.0) / 2.0),
        );
        let circle_2 = Circle2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            segment_2_circle_2_intersection_point_2(&segment_2, &circle_2, None),
            Some(vec![
                Point2::new(f64::sqrt(2.0) / -2.0, f64::sqrt(2.0) / 2.0),
                Point2::new(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
            ])
        );

        let segment_2 = Segment2::new(Point2::new(10.0, 10.0), Point2::new(-10.0, -10.0));
        let circle_2 = Circle2::new(Point2::new(0.0, 0.0), 1.0);
        assert_eq!(
            segment_2_circle_2_intersection_point_2(&segment_2, &circle_2, None),
            Some(vec![
                Point2::new(f64::sqrt(2.0) / -2.0, f64::sqrt(2.0) / -2.0),
                Point2::new(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
            ])
        );
    }
}
