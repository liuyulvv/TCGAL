use crate::{
    geometry::{point_2::Point2, segment_2::Segment2},
    traits::eps::Eps,
};

pub fn is_point_2_on_segment_2(point_2: &Point2, segment_2: &Segment2, eps: Option<Eps>) -> bool {
    let eps = eps.unwrap_or(Eps::default()).value;
    let ab = segment_2.end - segment_2.start;
    let ac = *point_2 - segment_2.start;
    let bc = *point_2 - segment_2.end;
    let dot_ab_ac = ab.dot(&ac);
    let dot_ab_ab = ab.dot(&ab);
    ab.cross(&ac).abs() < eps
        && ab.cross(&bc).abs() < eps
        && dot_ab_ac >= -eps
        && dot_ab_ac <= dot_ab_ab
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_point_2_on() {
        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.0);
        let s = Segment2::new(p1, p2);
        let result = is_point_2_on_segment_2(&Point2::new(1.0, 1.0), &s, None);
        assert_eq!(result, true);

        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let s = Segment2::new(p1, p2);
        let result = is_point_2_on_segment_2(&Point2::new(3.333333, 3.333333), &s, None);
        assert_eq!(result, true);
        let result = is_point_2_on_segment_2(&Point2::new(3.333334, 3.333333), &s, None);
        assert_eq!(result, false);
        let result = is_point_2_on_segment_2(&Point2::new(20.0, 20.0), &s, None);
        assert_eq!(result, false);
    }
}
