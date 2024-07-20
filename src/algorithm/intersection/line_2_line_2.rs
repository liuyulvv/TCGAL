use crate::kernel::{line_2::Line2, number_type::NumberType, point_2::Point2};

pub fn is_line_2_line_2_intersected<T: NumberType>(line_a: &Line2<T>, line_b: &Line2<T>) -> bool {
    let intersection = line_2_line_2_intersection(line_a, line_b);
    intersection.is_some()
}

pub fn line_2_line_2_intersection<T: NumberType>(
    line_a: &Line2<T>,
    line_b: &Line2<T>,
) -> Option<Point2<T>> {
    let a_1 = line_a.a();
    let b_1 = line_a.b();
    let c_1 = line_a.c();
    let a_2 = line_b.a();
    let b_2 = line_b.b();
    let c_2 = line_b.c();
    let det = a_1 * b_2 - a_2 * b_1;
    if det.equals(T::zero()) {
        return None;
    }
    let x = (-b_2 * c_1 + b_1 * c_2) / det;
    let y = (a_2 * c_1 - a_1 * c_2) / det;
    Some(Point2::new(x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_line_2_line_2_intersected() {
        let line_a = Line2::new(1.0, 1.0, 0.0);
        let line_b = Line2::new(1.0, -1.0, 0.0);
        let is_intersected = is_line_2_line_2_intersected(&line_a, &line_b);
        assert_eq!(is_intersected, true);

        let line_a = Line2::new(1.0, 1.0, 0.0);
        let line_b = Line2::new(1.0, 1.0, 0.0);
        let is_intersected = is_line_2_line_2_intersected(&line_a, &line_b);
        assert_eq!(is_intersected, false);

        let line_a = Line2::new(1.0, 1.0, 0.0);
        let line_b = Line2::new(1.0, 1.0, 1.0);
        let is_intersected = is_line_2_line_2_intersected(&line_a, &line_b);
        assert_eq!(is_intersected, false);

        let line_a = Line2::new(-1.0, 1.0, 0.0);
        let line_b = Line2::new(1.0, 1.0, -5.0);
        let is_intersected = is_line_2_line_2_intersected(&line_a, &line_b);
        assert_eq!(is_intersected, true);
    }

    #[test]
    fn test_line_2_line_2_intersection() {
        let line_a = Line2::new(1.0, 1.0, 0.0);
        let line_b = Line2::new(1.0, -1.0, 0.0);
        let intersection = line_2_line_2_intersection(&line_a, &line_b);
        assert_eq!(intersection, Some(Point2::new(0.0, 0.0)));

        let line_a = Line2::new(1.0, 1.0, 0.0);
        let line_b = Line2::new(1.0, 1.0, 0.0);
        let intersection = line_2_line_2_intersection(&line_a, &line_b);
        assert_eq!(intersection, None);

        let line_a = Line2::new(1.0, 1.0, 0.0);
        let line_b = Line2::new(1.0, 1.0, 1.0);
        let intersection = line_2_line_2_intersection(&line_a, &line_b);
        assert_eq!(intersection, None);

        let line_a = Line2::new(-1.0, 1.0, 0.0);
        let line_b = Line2::new(1.0, 1.0, -5.0);
        let intersection = line_2_line_2_intersection(&line_a, &line_b);
        assert_eq!(intersection, Some(Point2::new(2.5, 2.5)));
    }
}
