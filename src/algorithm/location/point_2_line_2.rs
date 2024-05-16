use crate::{
    geometry::{line_2::Line2, point_2::Point2},
    traits::eps::Eps,
};

pub fn is_point_2_on_line_2(point_2: &Point2, line_2: &Line2, eps: Option<Eps>) -> bool {
    let eps = eps.unwrap_or(Eps::default()).value;
    let result = line_2.a * point_2.x + line_2.b * point_2.y + line_2.c;
    return result.abs() < eps;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_point_2_on() {
        let p1 = Point2::new(1.0, 1.0);
        let p2 = Point2::new(1.0, 1.0);
        let s = Line2::from_points(p1, p2);
        let result = is_point_2_on_line_2(&Point2::new(1.0, 1.0), &s, None);
        assert_eq!(result, true);

        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let s = Line2::from_points(p1, p2);
        let result = is_point_2_on_line_2(&Point2::new(3.333333, 3.333333), &s, None);
        assert_eq!(result, true);
        let result = is_point_2_on_line_2(&Point2::new(3.333334, 3.333333), &s, None);
        assert_eq!(result, false);
        let result = is_point_2_on_line_2(&Point2::new(20.0, 20.0), &s, None);
        assert_eq!(result, true);
    }
}
