use crate::{
    geometry::{point_2::Point2, segment_2::Segment2},
    traits::eps::Eps,
};

pub fn point_2_project_segment_2(
    point_2: &Point2,
    segment_2: &Segment2,
    eps: Option<Eps>,
) -> Option<Point2> {
    let eps = eps.unwrap_or(Eps::default()).value;
    let v = segment_2.end - segment_2.start;
    let w = *point_2 - segment_2.start;
    let c1 = w.dot(&v);
    let c2 = v.dot(&v);
    if c1 < eps || c1 > c2 - eps {
        return None;
    }
    let b = c1 / c2;
    return Some(segment_2.start + Point2::from_vector(&v) * b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_2_segment_2() {
        let point_2 = Point2::new(1.0, 1.0);
        let segment_2 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(2.0, 2.0));
        let result = point_2_project_segment_2(&point_2, &segment_2, None);
        assert_eq!(result, Some(Point2::new(1.0, 1.0)));

        let point_2 = Point2::new(0.0, 2.0);
        let segment_2 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(2.0, 2.0));
        let result = point_2_project_segment_2(&point_2, &segment_2, None);
        assert_eq!(result, Some(Point2::new(1.0, 1.0)));
    }
}
