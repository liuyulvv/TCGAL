use crate::{
    geometry::{line_2::Line2, point_2::Point2},
    traits::{eps::Eps, is_same::IsSame},
};

/// overlay is considered as intersect
pub fn is_line_2_line2_intersected(line_2_a: &Line2, line_2_b: &Line2, eps: Option<Eps>) -> bool {
    let eps_value = eps.unwrap_or(Eps::default()).value;
    let det = line_2_a.a * line_2_b.b - line_2_a.b * line_2_b.a;
    if det.abs() < eps_value {
        return line_2_a.is_same(line_2_b, eps);
    }
    true
}

/// overlay is not implemented
pub fn line_2_line_2_intersection_point_2(
    segment_2_a: &Line2,
    segment_2_b: &Line2,
    eps: Option<Eps>,
) -> Option<Point2> {
    let eps_value = eps.unwrap_or(Eps::default()).value;
    let det = segment_2_a.a * segment_2_b.b - segment_2_a.b * segment_2_b.a;
    if det.abs() < eps_value {
        if segment_2_a.is_same(segment_2_b, eps) {
            // TODO: implement overlap
            return None;
        }
        return None;
    }
    let x = (segment_2_a.b * segment_2_b.c - segment_2_a.c * segment_2_b.b) / det;
    let y = (segment_2_a.c * segment_2_b.a - segment_2_a.a * segment_2_b.c) / det;
    return Some(Point2::new(x, y));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_intersect() {
        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 1.0, 1.0);
        let result = is_line_2_line2_intersected(&l1, &l2, None);
        assert_eq!(result, true);

        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 1.0, 2.0);
        let result = is_line_2_line2_intersected(&l1, &l2, None);
        assert_eq!(result, false);

        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 2.0, 1.0);
        let result = is_line_2_line2_intersected(&l1, &l2, None);
        assert_eq!(result, true);

        let l1 = Line2::new(1.0, 0.0, 1.0);
        let l2 = Line2::new(1.0, 0.0, 2.0);
        let result = is_line_2_line2_intersected(&l1, &l2, None);
        assert_eq!(result, false);

        let l1 = Line2::new(0.0, 1.0, 1.0);
        let l2 = Line2::new(0.0, 1.0, 2.0);
        let result = is_line_2_line2_intersected(&l1, &l2, None);
        assert_eq!(result, false);
    }

    #[test]
    fn intersection() {
        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 1.0, 1.0);
        let result = line_2_line_2_intersection_point_2(&l1, &l2, None);
        assert_eq!(result, None);

        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 1.0, 2.0);
        let result = line_2_line_2_intersection_point_2(&l1, &l2, None);
        assert_eq!(result, None);

        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 2.0, 1.0);
        let result = line_2_line_2_intersection_point_2(&l1, &l2, None);
        assert_eq!(result, Some(Point2::new(-1.0, 0.0)));

        let l1 = Line2::new(1.0, 1.0, 1.0);
        let l2 = Line2::new(1.0, 2.0, 2.0);
        let result = line_2_line_2_intersection_point_2(&l1, &l2, None);
        assert_eq!(result, Some(Point2::new(0.0, -1.0)));

        let l1 = Line2::new(1.0, 0.0, 1.0);
        let l2 = Line2::new(1.0, 0.0, 2.0);
        let result = line_2_line_2_intersection_point_2(&l1, &l2, None);
        assert_eq!(result, None);
    }
}
