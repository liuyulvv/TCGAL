use crate::{
    geometry::{circle_2::Circle2, point_2::Point2},
    traits::eps::Eps,
};

use super::location_enum::Point2Circle2Relation;

pub fn is_point_2_on_circle_2(point_2: &Point2, circle_2: &Circle2, eps: Option<Eps>) -> bool {
    let eps = eps.unwrap_or(Eps::default()).value;
    let center = circle_2.center;
    let radius = circle_2.radius;
    let distance = point_2.distance_to(&center);
    (distance - radius).abs() < eps
}

pub fn is_point_2_inside_circle_2(point_2: &Point2, circle_2: &Circle2, eps: Option<Eps>) -> bool {
    let eps = eps.unwrap_or(Eps::default()).value;
    let center = circle_2.center;
    let radius = circle_2.radius;
    let distance = point_2.distance_to(&center);
    distance < radius + eps
}

pub fn is_point_2_outside_circle_2(point_2: &Point2, circle_2: &Circle2, eps: Option<Eps>) -> bool {
    let eps = eps.unwrap_or(Eps::default()).value;
    let center = circle_2.center;
    let radius = circle_2.radius;
    let distance = point_2.distance_to(&center);
    distance > radius - eps
}

pub fn point_2_relation_to_circle_2(
    point_2: &Point2,
    circle_2: &Circle2,
    eps: Option<Eps>,
) -> Point2Circle2Relation {
    let eps = eps.unwrap_or(Eps::default()).value;
    let center = circle_2.center;
    let radius = circle_2.radius;
    let distance = point_2.distance_to(&center);
    if (distance - radius).abs() < eps {
        Point2Circle2Relation::ON
    } else if distance <= radius + eps {
        Point2Circle2Relation::INSIDE
    } else {
        Point2Circle2Relation::OUTSIDE
    }
}
