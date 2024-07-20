use crate::{
    algorithm::intersection::line_2_line_2::line_2_line_2_intersection,
    kernel::{line_2::Line2, number_type::NumberType, point_2::Point2},
};

pub fn point_2_project_line_2<T: NumberType>(point: &Point2<T>, line: &Line2<T>) -> Point2<T> {
    let a = line.a();
    let b = line.b();
    if b.equals(T::zero()) {
        let c = line.c();
        let x = -c / a;
        return Point2::new(x, point.y());
    } else if a.equals(T::zero()) {
        let c = line.c();
        let y = -c / b;
        return Point2::new(point.x(), y);
    } else {
        let t = a / b;
        let c = point.y() - t * point.x();
        let line_vertical = Line2::new(t, T::from_f64(-1.0), c);
        let intersection = line_2_line_2_intersection(line, &line_vertical);
        return intersection.unwrap();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_point_2_project_line_2() {
        let point = Point2::new(1.0, 1.0);
        let line = Line2::new(1.0, 1.0, 0.0);
        let projection = point_2_project_line_2(&point, &line);
        assert_eq!(projection, Point2::new(0.0, 0.0));

        let point = Point2::new(1.0, 1.0);
        let line = Line2::new(1.0, 0.0, 0.0);
        let projection = point_2_project_line_2(&point, &line);
        assert_eq!(projection, Point2::new(0.0, 1.0));

        let point = Point2::new(1.0, 1.0);
        let line = Line2::new(0.0, 1.0, 0.0);
        let projection = point_2_project_line_2(&point, &line);
        assert_eq!(projection, Point2::new(1.0, 0.0));
    }
}
